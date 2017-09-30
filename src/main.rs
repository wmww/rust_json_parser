use std::str;

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

fn parse_i(mut iter: std::str::Chars) -> (i32, std::str::Chars) {
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
        let number = (u_part.0 as i32) * (if negative { -1 } else { 1 });
        return (number, u_part.1);
    } else {
        return (0, iter);
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

fn parse_f(mut iter: str::Chars) -> (f64, str::Chars) {
    let mut next = iter.clone();
    let mut negative = false;
    if let Some(c) = next.next() {
        if c == '-' {
            negative = true;
            iter = next;
        } else if c == '+' {
            iter = next;
        }
        let (u_part, mut next) = parse_u(iter);
        iter = next.clone();
        let mut number = u_part as f64;
        if let Some(c) = next.next() {
            if c == '.' {
                let (decimal_part, mut next) = parse_decimal(next);
                number += decimal_part;
                iter = next.clone();
                if let Some(c) = next.next() {
                    if c == 'e' || c == 'E' {
                        let (e, next) = parse_i(next);
                        number *= 10f64.powi(e) as f64;
                        iter = next;
                    }
                }
            }
        }
        return (number * (if negative { -1.0 } else { 1.0 }), iter);
    } else {
        return (0.0, iter);
    }
}

fn main() {
    let json_text = "-34.889e-2";
    
    //let iter = json_text.chars();
    //parse_num(iter);
    println!("number: {}", parse_f(json_text.chars()).0);
    /*
    while let c: char = iter.next()
    {
        println!("{}", c.unwrap());
    }
    */
}
