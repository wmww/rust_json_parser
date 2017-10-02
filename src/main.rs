use std::str;

enum JsonValue {
    Array(Vec<JsonValue>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl JsonValue {
    fn to_string(self) -> String {
        match self {
            JsonValue::Array(v) => String::from("[...]"),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Bool(b) => String::from(if b { "true" } else { "false" }),
            JsonValue::Null => String::from("null"),
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

fn parse_json_array(mut iter: str::Chars) -> (Vec<JsonValue>, str::Chars) {
    if iter.next() != Some('[') {
        print_error("parse_json_array called on non array");
        return (Vec::new(), iter);
    }
    let mut data = Vec::<JsonValue>::new();
    loop {
        iter = skip_whitespace(iter);
        let prev = iter.clone();
        match iter.next() {
            Some(']') | None => return (data, iter),
            _ => {
                let (value, end_iter) = parse_json_value(prev);
                data.push(value);
                iter = end_iter;
            }
        }
    }
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

fn parse_json_value(mut iter: str::Chars) -> (JsonValue, str::Chars) {
    iter = skip_whitespace(iter);
    let prev = iter.clone();
    match iter.next() {
        Some('{') => {
            return (JsonValue::Null, iter);
        },
        Some('[') => {
            let data = parse_json_array(prev);
            return (JsonValue::Array(data.0), data.1);
        },
        Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
            let data = parse_json_number(prev);
            return (JsonValue::Number(data.0), data.1);
        },
        Some('"') => {
            let data = parse_json_string(prev);
            return (JsonValue::String(data.0), data.1);
        },
        Some('t') => {
            return (JsonValue::Null, iter);
        },
        Some('f') => {
            return (JsonValue::Null, iter);
        },
        Some('n') => {
            return (JsonValue::Null, iter);
        },
        Some(_) | None => {
            println!("bad token");
            return (JsonValue::Null, iter);
        }
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
    let json_text = "[-57.9, 4.3]";
    let (value, mut next) = parse_json_value(json_text.chars());
    println!("value: {}, next: {}", value.to_string(), if let Some(c) = next.next() { c } else { '$' });
}
