
// Ownership and copy types
// It's trivial to copy the bytes
// i, u ....

fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_string(input:String){
    println!("{}", input);
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Austria".to_string();

    //copy -> copy types
    //clone -> non-copy types
    prints_string(my_country.clone()); //메모리 떄문에 없애는게 좋음 ref권장
    prints_string(my_country);
}
