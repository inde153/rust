fn main() {
    print!("This");
    print!("This");

    print!("this\ndrive\nnew\ndrive\n");
    print!(r#"c:\thisdrive\new_drive"#);

    println!("
Let me tell you
어떤 이야기를
봅시다.");

    let my_variable = &9000;
    println!("{:x}",my_variable); //16진수 헥사데시멀
    println!("{:X}",my_variable); //16진수 헥사데시멀 대문자
    println!("{:p}",my_variable); //reference
    println!("{:b}",my_variable); //reference

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
}