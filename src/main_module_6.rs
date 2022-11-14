

fn main_module_6(){
    if true {
        let planet = "Earth";
        println!("planet it {planet}");
    }

    let mut message = String::from("Earth");
    message.push_str("is home");
    message += "+=";
    println!("{message}");

    let mut rocket_fuel = String::from("123");
    rocket_fuel = fun(rocket_fuel);
    println!("{rocket_fuel}");
}

fn fun(mut proppelent: String) -> String {
    proppelent += "456";
    return proppelent;
}