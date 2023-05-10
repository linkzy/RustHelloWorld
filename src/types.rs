pub fn run(){
    // i32
    let x = 1;

    // f64
    let y = 2.5;

    //Explicit type
    let z: i64 = 999999;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    //Get boolean from expression
    let is_greater = 10 > 5;

    //char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}