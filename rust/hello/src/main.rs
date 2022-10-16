fn main() {
    let a = 10;
    let b = 15;
    println!("Hello, world! {} {}", a, b);

    let unsigned: u8 = 100;
    let signed: i8 = -100;
    let char = "c";
    let is_true: bool = true || false;
    println!("Number !! {} {}", unsigned, signed);
    println!("Istrue: {}", is_true);
    println!("Character: {}", char);

    //array
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let other_arr: [u8; 5] = [100; 5];
    println!("index {}, arr_length: {}", arr[0], arr.len());
    println!("{:?}", other_arr);

    //tuple
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    let (d, e, f) = tuple;
    println!("first {}, second {}, third {}", d, e, f);
    println!("{:?}", tuple2);

    // func
    println!("is even: {}", is_even(8));

    // mutable
    let mut num = 5;
    num += 3;
    println!("Num :{}", num);

    // array and slices
    let arr: [u8; 4] = [0, 1, 2, 3];
    let slice2: &[u8] = &arr[1..3]; //[1,2] => don't need to know the length
    println!("slice: {:?}", slice2);
    // string
    let s: &str = "Hello world";
    let mut string: String = String::from("Hello World");
    let slice = &string[..6];
    println!("Slice string lenght: {:?}", slice.len());
    string += s;
    string.push('1');
    string.push_str("Kewin 1807");
    string = string.replace("Hello", "Bye");
    println!("Test string: {:?}", string);

    //if_else_statement
    if_else_statement();

    // for_loop
    for_loop();
    // while_loop
    while_loop();

    // match statement (switch case)
    match_statement();

    //Struct
    
}
pub fn is_even(num: u8) -> bool {
    let degit: u8 = num % 2;
    return degit == 0;
}

pub fn if_else_statement() {
    let n = 3;
    if n % 2 == 1 {
        println!("this number is odd!!");
    } else {
        println!("this number is even!!!");
    }
}

pub fn for_loop() {
    for i in 0..6 {
        println!("{:?}", i) // print 0 -> 5
    }
}

pub fn while_loop() {
    let mut i = 0;
    while i < 4 {
        println!("Number: {} ", i);
        i += 1;
    }
}

pub fn match_statement() {
    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1 || 2"),
        3..=4 => println!("3 ... 4"),
        _ => println!("default"),
    }
}
