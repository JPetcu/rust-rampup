


fn main_module_3() {
    println!("Module 3");
    let _letters = ['a', 'b', 'c'];

    let numbers: [i32; 5];
    numbers = [0 ; 5];
    let index = numbers.len()-1;
    println!("last number is {}", numbers[index]);

    let parking_lot = [[1, 2, 3], [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);


    let stuff = (10, 3.14, 'x');
    let first_item = stuff.0;
    let second_item = stuff.1;

    println!("first item is:{first_item}\nsecond item is {second_item}");

    let (int, float, character) = stuff;

    println!("the float is: {float}");

}