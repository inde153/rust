//macro = function that writes code
fn give_age() -> i32 {
    let my_age = 42;
    my_age
    //return my_age;
}

fn main(){

    let my_name = "kim";

    println!("My name is {} and my age is {}", my_name, give_age())
}