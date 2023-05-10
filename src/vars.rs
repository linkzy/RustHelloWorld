pub fn run(){
    let name = "Dan";
    let mut age = 35;
    println!("My name is {} and i am {}", name, age);
    
    age = 36;
    println!("My name is {} and i am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables
    let(my_name, my_age) = ("Dan", 35);
    println!("{} and {}", my_name, my_age);

}