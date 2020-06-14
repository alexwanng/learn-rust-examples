/// parsing a String

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();


    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

fn int_value() -> i32 {
    100i32
}

#[test]
fn test_hello() {
    assert_eq!(int_value(), 100i32);
}