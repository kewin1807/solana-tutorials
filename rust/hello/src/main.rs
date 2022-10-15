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
    num = 3;
    println!("Num :{}", num)

    // array and slices
    
}
pub fn is_even(num:u8) -> bool{
    let degit: u8 = num %2;
    return degit == 0;
}