




fn celsius_to_fahrenheit(celsius : f64) -> f64 {

    return (1.8 * celsius) + 32.0;
}
fn main_module_4(){
   
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");





}