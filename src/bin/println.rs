//macro = function that writes code
fn give_age() -> i32 {
    let my_age = 42;
    my_age
    //return my_age;
}


fn main(){
    
    let my_name = "kim";
    let year = 2002;
    let population = 100_000_000;
    let my_city = "Seoul";
    
    println!("My name is {} and my age is {}", my_name, give_age());
    
    //string interpolation
    println!("The city of {my_city} in {year} had a population of {population}");


    println!("The city of {0} in {1} had a population of {2}. I Love {0}", my_city,year,population);

    //expression
    println!("The city of {0} in {1} had a population of {2}. I Love {0}", my_city,year + 2,population);
}