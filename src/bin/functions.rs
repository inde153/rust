fn give_number (one: i32, two: i32) -> i16 {
   let multiplied_by_ten = {
    let first_number = 10;
    first_number * one * two
   };
   multiplied_by_ten as i16
}

fn main() {
    let my_number = give_number(9,1);
    println!("{}", my_number)
}
