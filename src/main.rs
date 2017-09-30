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

/*
fn parse_f(mut iter: std::str::Chars) -> (f64, std::str::Chars) {
    let mut number: f64 = 0.0; // the accumulated output number
    let mut decimal_pos = 0; // places we are to the right of the decimal, zero if we are to the left
    let mut is_start = true; // if this is the first iteration
    let mut is_valid = false; // if the number is currently in a valid state
    let mut negative = false; // if the output number is negative
    loop {
        let next = iter.clone
        if let Some(c) = next.next() {
            if c == '-' {
                if is_start {
                    negative = true;
                } else {
                    break;
                }
            } else if 
        } else {
            break;
        }
        is_start = false;
        iter = next;
    }
    return (number, iter);
}
*/

fn main() {
    let json_text = "1788afdai";
    
    //let iter = json_text.chars();
    //parse_num(iter);
    println!("number: {}", parse_i(json_text.chars()).0);
    /*
    while let c: char = iter.next()
    {
        println!("{}", c.unwrap());
    }
    */
}
