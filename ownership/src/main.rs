fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    //s.push_str(", world");
    let s3 = takes_ownership(&mut s2);
    println!("{}", s3);
    
    let x = 5;
    let y = x;
    makes_copy(y);
    println!("{}", y);

    let s = dangle();
    println!("{}", s);

    let len = first_word(s3);
    println!("{}", len);
}

fn takes_ownership(some_string: &mut String) -> &mut String {
    some_string.push_str(", world!");
    some_string
}

fn makes_copy(some_integer: i32) -> i32 {
    println!("{}",some_integer);
    some_integer
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}abc",item);
            return &s[0..i];
        }
    }
    &s[0..s.len()]
}
