pub fn run(){
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");

    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("Contains {}", hello.contains("Wor"));
    println!("Contains {}", hello.contains("wor"));

    println!("Replace {}", hello.replace("Wor", "wor"));

    hello = String::from("thats it!");

    for word in hello.split_whitespace(){
        println!("{}", word);    
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
