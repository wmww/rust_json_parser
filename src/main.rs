use std::str;
use std::collections::HashMap;

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

enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Invalid(String),
}

impl JsonValue {
    fn to_string(&self) -> String {
        match self {
            &JsonValue::Object(ref o) => format!("(...)"),
            &JsonValue::Array(ref v) => format!("{}{}{}", "[", v.iter().fold(String::new(), |sum, x| format!("{}{}{}", sum, if sum == String::from("") { "" } else { ", " }, x.to_string())), "]"),
            &JsonValue::Number(ref n) => format!("{}", n.to_string()),
            &JsonValue::String(ref s) => format!("\"{}\"", s),
            &JsonValue::Bool(ref b) => format!("{}", if *b { "true" } else { "false" }),
            &JsonValue::Null => String::from("null"),
            &JsonValue::Invalid(ref text) => format!("(INVALID VALUE '{}')", text),
        }
    }
}

fn print_error(message: &str) {
    println!("error: {}", message);
}

// I am aware of the parse function in the standard lib, but I am not using it because
// 1. learning, 2. fun and 3. it might not be completely compatible with JSON
fn parse_json_number(mut iter: str::Chars) -> (f64, str::Chars) {
    
    fn parse_u(mut iter: std::str::Chars) -> (u32, std::str::Chars) {
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
    fn parse_i(mut iter: std::str::Chars) -> (i32, bool, std::str::Chars) {
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
    fn parse_decimal(mut iter: str::Chars) -> (f64, str::Chars) {
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
    if let Some(c) = next.next() {
        if c == '.' {
            let (decimal_part, mut next) = parse_decimal(next);
            number += decimal_part * (if negative { -1.0 } else { 1.0 });
            iter = next.clone();
            if let Some(c) = next.next() {
                if c == 'e' || c == 'E' {
                    let (e, _, next) = parse_i(next);
                    number *= 10f64.powi(e) as f64;
                    iter = next;
                }
            }
        }
    }
    return (number, iter);
}

fn parse_json_string(mut iter: str::Chars) -> (String, str::Chars) {
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
                    Some(_) => '?',
                    None => break,
                };
                text.push(c);
            }
            Some(c) => text.push(c),
        }
    }
    return (text, iter);
}

fn skip_whitespace(mut iter: str::Chars) -> str::Chars {
    loop {
        let prev = iter.clone();
        match iter.next() {
            Some(' ') | Some('\t') | Some('\n') => continue,
            Some(_) | None => return prev,
        }
    }
}

fn next_token(mut iter: str::Chars) -> (JsonToken, str::Chars) {
    iter = skip_whitespace(iter);
    let prev = iter.clone();
    (match iter.next() {
        Some('{') => JsonToken::ObjectOpen,
        Some('}') => JsonToken::ObjectClose,
        Some('[') => JsonToken::ArrayOpen,
        Some(']') => JsonToken::ArrayClose,
        Some(',') => JsonToken::Comma,
        Some(':') => JsonToken::Colon,
        Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
            let data = parse_json_number(prev);
            iter = data.1;
            JsonToken::Number(data.0)
        },
        Some('"') => {
            let data = parse_json_string(prev);
            iter = data.1;
            JsonToken::String(data.0)
        },
        Some(c) => {
            let mut text = String::new();
            text.push(c);
            loop {
                let prev = iter.clone();
                let c = iter.next();
                if c.is_none() || !c.unwrap().is_alphabetic() {
                    iter = prev;
                    break;
                } else {
                    text.push(c.unwrap());
                }
            }
            JsonToken::Identifier(text)
        },
        None => JsonToken::End,
    }, iter)
}

fn parse_json_array(mut iter: str::Chars) -> (Option<Vec<JsonValue>>, str::Chars) {
    let (start_token, mut iter) = next_token(iter);
    match start_token {
        JsonToken::ArrayOpen => (),
        _ => return (None, iter),
    }
    let mut data = Vec::<JsonValue>::new();
    loop {
        let (token, next) = next_token(iter.clone());
        match token {
            JsonToken::ArrayClose => return (Some(data), next),
            JsonToken::Comma if !data.is_empty() => { iter = next },
            JsonToken::End => {
                data.push(JsonValue::Invalid(token.to_string()));
                return (Some(data), next);
            }
            _ if !data.is_empty() => data.push(JsonValue::Invalid(token.to_string())),
            _ => (),
        }
        let (value, next) = parse_json_value(iter);
        data.push(value);
        iter = next;
    }
}

fn parse_json_value(mut iter: str::Chars) -> (JsonValue, str::Chars) {
    let (token, mut next) = next_token(iter.clone());
    match token {
        JsonToken::ArrayOpen => {
            let (data, iter) = parse_json_array(iter);
            return (
                match data{
                    Some(data) => JsonValue::Array(data),
                    None => JsonValue::Invalid("[array fuckup]".to_string()),
                }
            , iter);
        },
        JsonToken::ArrayClose
            | JsonToken::ObjectOpen
            | JsonToken::ObjectOpen
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

fn main() {
    //let json_text = "9.25e-2abc";
    //let (num, mut next) = parse_json_number(json_text.chars());
    //println!("number: {}, next: {}", num, if let Some(c) = next.next() { c } else { '$' });
    
    //let json_text = "\"test\\n\\\"string\"a";
    //let (text, mut next) = parse_json_string(json_text.chars());
    //println!("text: '{}', next: {}", text, if let Some(c) = next.next() { c } else { '$' });
    
    //let json_text = "\"test\\n\\\"string\"a";
    let json_text = "[89.3, true, {}, [\"hey\", [79.3, null, -49.221e-2], false]]";
    let (value, mut next) = parse_json_value(json_text.chars());
    println!("value: {}, next: {}", value.to_string(), if let Some(c) = next.next() { c } else { '$' });
}

