//() - empty tuple, unit type (void)
//expression-based language

fn empty_tuple() -> () {
    8;
}

fn main() {
    let tuple = empty_tuple();

    // Display {}
    // println!("{}", tuple);

    //Debug print
    println!("{:?}", tuple);
}
