



fn main_module_2() {
    let mut x = 255.0;
    println!("x is {}", x);
    x = x + 1.0;
    println!("x is {}", x);

    let a = 10.0;
    let b = 3.0;
    let c = a / b;

    println!("c is {0:08.3}\na is {1:08}\nonce again, c is {0:.3}\nand the third time c is {c:08.3}", c, a);
    let mut value = 0b1111_0101;
    println!("value is {value:08b}");

    value = !value;

    println!("value is {value:08b}");

    value = value & 0b1111_0110;

    println!("value is {value:08b}");
    println!("bit 1 is: {}", value & 0b0000_0100);

    value = 2;
    value = value << 7;
    println!("value is: {value}");

    println!();
    println!();

    let _letter = 'a';
    let _number = '1';

    let a = 13;
    let b = 2.3 ;
    let c: f32 = 120.0;



    let average = (a as f64 + b + c as f64)/3.0;
    assert_eq!(average, 45.1);
    println!("TEST PASSED");

    
}
