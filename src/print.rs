pub fn run(){
    //Just print to console
    println!("Hello world form the print file!");

    //Basic formatting
    println!("Number: {}, {}, {}", 1, 2, 3);

    //Positional Args
    println!("{0} is in the first postions, {1} is in the second position, {0} is also in the third position", "one", "two");

    //Named Args
    println!("First name: {name} - Last name: {last}", name="Danilo", last="Cavalcante");

    //Placeholder traits
    println!("Binatry: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10+10 = {}", 10+10);
}