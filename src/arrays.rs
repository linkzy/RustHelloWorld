pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    numbers[3] = 99;

    println!("{:?}", numbers);
    println!("Single value: {}", numbers[0]);
    println!("Len: {}", numbers.len());
    println!("size:: {}", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

}