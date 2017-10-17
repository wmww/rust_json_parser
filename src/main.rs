mod wmww_json;
use wmww_json::*;

fn main() {
    //let json_text = "9.25e-2abc";
    //let (num, mut next) = parse_json_number(json_text.chars());
    //println!("number: {}, next: {}", num, if let Some(c) = next.next() { c } else { '$' });

    //let json_text = "\"test\\n\\\"string\"a";
    //let (text, mut next) = parse_json_string(json_text.chars());
    //println!("text: '{}', next: {}", text, if let Some(c) = next.next() { c } else { '$' });

    //let json_text = "\"test\\n\\\"string\"a";
    let json_text = "[89.3, true, {\"test\": true, \"2\"    :   49.3}, [\"hey\", [79.3, 57e22, null, -49.221e-2], false]]";
    let value = parse_json(json_text);
    println!("content: {}", value.to_string());
}
