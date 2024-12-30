fn add_is_great(country_name : &mut String){
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

//take by value, declare as mutable
fn add_is_great2(mut country_name : String){
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name); 
}

fn main() {
    let mut my_country = "캐나다".to_string();

    // add_is_great(my_country); by mutable value
    add_is_great(&mut my_country); // 

    let my_country = "대한민국".to_string();
    add_is_great2(my_country); 
}
