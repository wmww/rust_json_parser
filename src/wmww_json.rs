
use std::str;
use std::str::Chars;
use std::collections::HashMap;

pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Invalid(String),
}

impl JsonValue {
    pub fn to_string(&self) -> String {
        self.to_string_with_indent("", "  ")
    }

    fn to_string_with_indent(&self, indent: &str, next_indent: &str) -> String {
        match self {
            &JsonValue::Object(ref o) => format!(
                "{{{}\n{}}}",
                o.iter().fold(String::new(), |sum, x| {
                    let next = format!("{}{}", indent, next_indent);
                    format!(
                        "{}{}\n{}{}: {}",
                        sum,
                        if sum == String::from("") { "" } else { ", " },
                        next,
                        x.0,
                        x.1.to_string_with_indent(&next, next_indent),
                    )
                }),
                indent,
            ),
            &JsonValue::Array(ref v) => format!(
                "[{}\n{}]",
                v.iter().fold(String::new(), |sum, x| {
                    let next = format!("{}{}", indent, next_indent);
                    format!(
                        "{}{}\n{}{}",
                        sum,
                        if sum == String::from("") { "" } else { ", " },
                        next,
                        x.to_string_with_indent(&next, next_indent),
                    )
                }),
                indent,
            ),
            &JsonValue::Number(ref n) => format!("{}", n.to_string()),
            &JsonValue::String(ref s) => format!("\"{}\"", s),
            &JsonValue::Bool(ref b) => format!("{}", if *b { "true" } else { "false" }),
            &JsonValue::Null => String::from("null"),
            &JsonValue::Invalid(ref text) => format!("(INVALID VALUE '{}')", text),
        }
    }
}

enum JsonToken {
    ArrayOpen,
    ArrayClose,
    Comma,
    ObjectOpen,
    ObjectClose,
    Colon,
    Number(f64),
    String(String),
    Identifier(String),
    Invalid(String),
    End,
}

impl JsonToken {
    fn to_string(&self) -> String {
        match self {
            &JsonToken::ArrayOpen => "[".to_string(),
            &JsonToken::ArrayClose => "]".to_string(),
            &JsonToken::Comma => ",".to_string(),
            &JsonToken::ObjectOpen => "{".to_string(),
            &JsonToken::ObjectClose => "}".to_string(),
            &JsonToken::Colon => ":".to_string(),
            &JsonToken::Number(ref n) => n.to_string(),
            &JsonToken::String(ref s) => format!("\"{}\"", s),
            &JsonToken::Identifier(ref s) => s.to_string(),
            &JsonToken::Invalid(ref text) => format!("<INVALID VALUE '{}'>", text),
            &JsonToken::End => "<END>".to_string(),
        }
    }
}

// I am aware of the parse function in the standard lib, but I am not using it because
// 1. learning, 2. fun and 3. it might not be completely compatible with JSON
fn parse_json_number(mut iter: Chars) -> (f64, Chars) {

    fn parse_u(mut iter: Chars) -> (u32, Chars) {
        let mut number: u32 = 0;
        loop {
            let mut next = iter.clone();
            if let Some(c) = next.next() {
                if let Some(digit) = c.to_digit(10) {
                    number = number * 10 + digit;
                } else {
                    break;
                }
            } else {
                break;
            }
            iter = next;
        }
        return (number, iter);
    }

    // the bool slot is if it is negative
    // (important because -0 is different from 0 if this is one component in parsing a decimal)
    fn parse_i(mut iter: Chars) -> (i32, bool, Chars) {
        let mut next = iter.clone();
        let mut negative = false;
        if let Some(c) = next.next() {
            if c == '-' {
                negative = true;
                iter = next;
            } else if c == '+' {
                iter = next;
            }
            let u_part = parse_u(iter);
            return (u_part.0 as i32 * (if negative { -1 } else { 1 }), negative, u_part.1);
        } else {
            return (0, false, iter);
        }
    }

    // parses a number assumed to be after the decimal point
    fn parse_decimal(mut iter: Chars) -> (f64, Chars) {
        let mut number = 0.0;
        let mut divider = 10.0;
        loop {
            let mut next = iter.clone();
            if let Some(c) = next.next() {
                if let Some(digit) = c.to_digit(10) {
                    number += digit as f64 / divider;
                    divider *= 10.0;
                } else {
                    break;
                }
            } else {
                break;
            }
            iter = next;
        }
        return (number, iter);
    }

    let (i_part, negative, mut next) = parse_i(iter);
    iter = next.clone();
    let mut number = i_part as f64;
    if let Some('.') = next.next() {
        let (decimal_part, next) = parse_decimal(next);
        number += decimal_part * (if negative { -1.0 } else { 1.0 });
        iter = next.clone();
    }
    let mut next = iter.clone();
    match next.next() {
        Some(c) if c == 'e' || c == 'E' => {
            let (e, _, next) = parse_i(next);
            number *= 10f64.powi(e) as f64;
            iter = next;
        },
        _ => (),
    }
    return (number, iter);
}

fn parse_json_string(mut iter: Chars) -> (String, Chars) {
    let start = iter.clone();
    let mut text = String::new();
    if let Some('"') = iter.next() {} else {
        return (String::from(""), start);
    }
    loop {
        match iter.next() {
            None => break,
            Some('"') => break,
            Some('\\') => {
                let c = match iter.next() {
                    Some('"') => '"',
                    Some('\\') => '\\',
                    Some('/') => '/',
                    Some('b') => '\x08',
                    Some('f') => '\x0C',
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some(_) => '�', // standard unicode replacement character
                    None => break,
                };
                text.push(c);
            }
            Some(c) => text.push(c),
        }
    }
    return (text, iter);
}

fn skip_whitespace(mut iter: Chars) -> Chars {
    loop {
        let prev = iter.clone();
        match iter.next() {
            Some(' ') | Some('\t') | Some('\n') => continue,
            Some(_) | None => return prev,
        }
    }
}

fn next_token(mut iter: Chars) -> (JsonToken, Chars) {
    iter = skip_whitespace(iter);
    let prev = iter.clone();
    (match iter.next() {
        Some('{') => JsonToken::ObjectOpen,
        Some('}') => JsonToken::ObjectClose,
        Some('[') => JsonToken::ArrayOpen,
        Some(']') => JsonToken::ArrayClose,
        Some(',') => JsonToken::Comma,
        Some(':') => JsonToken::Colon,
        Some(c) if c.is_digit(10) || c == '-' => { // + intentionally excluded
            let data = parse_json_number(prev);
            iter = data.1;
            JsonToken::Number(data.0)
        },
        Some('"') => {
            let data = parse_json_string(prev);
            iter = data.1;
            JsonToken::String(data.0)
        },
        Some(c) if c.is_alphabetic() => {
            let mut text = c.to_string();
            let mut next = iter.clone();
            loop {
                match next.next() {
                    Some(c) if c.is_alphabetic() => {
                        text.push(c);
                        iter = next.clone();
                    },
                    _ => break,
                }
            }
            JsonToken::Identifier(text)
        },
        None => JsonToken::End,
        Some(c) => JsonToken::Invalid(c.to_string()),
    }, iter)
}

fn parse_json_array(iter: Chars) -> (Vec<JsonValue>, Chars) {
    let (start_token, mut iter) = next_token(iter);
    let mut data = Vec::<JsonValue>::new();
    match start_token {
        JsonToken::ArrayOpen => (),
        _ => {
            data.push(JsonValue::Invalid("bad array start character".to_owned()));
            return (data, iter)
        },
    }
    loop {
        let (token, next) = next_token(iter.clone());
        match token {
            JsonToken::ArrayClose => return (data, next),
            JsonToken::Comma if !data.is_empty() => { iter = next },
            JsonToken::End => {
                data.push(JsonValue::Invalid(token.to_string()));
                return (data, next);
            }
            _ if !data.is_empty() => data.push(JsonValue::Invalid(token.to_string())),
            _ => (),
        }
        let (value, next) = parse_json_value(iter);
        data.push(value);
        iter = next;
    }
}

fn parse_json_object(iter: Chars) -> (HashMap<String, JsonValue>, Chars) {
    let (start_token, mut iter) = next_token(iter);
    let mut data = HashMap::<String, JsonValue>::new();
    match start_token {
        JsonToken::ObjectOpen => (),
        _ => {
            data.insert("INVALID MAP".to_owned(), JsonValue::Invalid("bad map start character".to_owned()));
            return (data, iter);
        },
    }
    loop {
        let (token, next) = next_token(iter.clone());
        match token {
            JsonToken::ObjectClose => { return (data, next); },
            JsonToken::Comma if !data.is_empty() => { iter = next.clone(); },
            JsonToken::End => {
                data.insert(token.to_string(), JsonValue::Invalid(token.to_string()));
                return (data, next);
            }
            _ if !data.is_empty() => { data.insert(token.to_string(), JsonValue::Invalid(token.to_string())); },
            _ => (),
        }
        let (key, next) = parse_json_value(iter);
        let (colon, next) = next_token(next);
        match colon {
            JsonToken::Colon => (),
            _ => {
                data.insert("INVALID".to_owned(), JsonValue::Invalid(colon.to_string()));
                iter = next;
                continue;
            }
        }
        let (value, next) = parse_json_value(next);
        match key {
            JsonValue::String(s) => { data.insert(s, value); },
            _ => { data.insert(key.to_string(), JsonValue::Invalid(key.to_string())); },
        }
        iter = next;
    }
}

fn parse_json_value(iter: Chars) -> (JsonValue, Chars) {
    let (token, next) = next_token(iter.clone());
    match token {
        JsonToken::ArrayOpen => {
            let (data, iter) = parse_json_array(iter);
            return (JsonValue::Array(data), iter);
        },
        JsonToken::ObjectOpen => {
            let (data, iter) = parse_json_object(iter);
            return (JsonValue::Object(data), iter);
        }
        JsonToken::ArrayClose
            | JsonToken::ObjectClose
            | JsonToken::Colon
            | JsonToken::Comma
            | JsonToken::End
            => return (JsonValue::Invalid(token.to_string()), next),
        JsonToken::Number(n) => return (JsonValue::Number(n), next),
        JsonToken::String(s) => return (JsonValue::String(s), next),
        JsonToken::Identifier(id) => {
            match id.as_ref() {
                "true" => return (JsonValue::Bool(true), next),
                "false" => return (JsonValue::Bool(false), next),
                "null" => return (JsonValue::Null, next),
                text => return (JsonValue::Invalid(text.to_string()), next),
            }
        },
        JsonToken::Invalid(text) => (JsonValue::Invalid(text), next),
    }
}

pub fn parse_json(json: &str) -> JsonValue {
    parse_json_value(json.chars()).0
}
