





fn main_module_5(){
    let x = 3;
    if x == 3
    {
        println!("X is {x}");
    }

    let y = 5;

    if x > y {
        println!("x is greater than y");
    }
    else if x == y {
        println!("x is equal to y");
    }
    else {
        println!("x is not greater than y");
    }

    //////////////////////////////////////////////////////////
    
    let mut count = 0;

    let result = loop {
        count +=1;
        println!("count is {count}");
        if count == 50{
            break count * 10;
        }
    };
    println!("Result is: {result}");

    while count < 100 {
        count += 1;
        println!("count is {count}");
    }

    count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }

    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate(){
        println!("item is {} at pos {index}", item);
    }

    for number in 0..5{
        print!("{number} ");
    }
    println!("\n");
    let mut matrix = [[1, 2, 3],
                   [4, 5, 6],
                   [7, 8, 9]];

    for row in matrix.iter_mut()
    {
        for num in row.iter_mut(){
            *num += 10;
            print!("{}\t",num);
        }
        println!();
    }


    println!("Challenge");

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut  mean: f64 = 0.0;


    for number in numbers{
        if max < number{
            max = number;
        }
        if min > number{
            min = number;
        }
        mean += number as f64;
    }
    mean = mean / (numbers.len() as f64) ;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test Passed");

}