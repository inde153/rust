// mutability
// shadowing -> 같은 이름을 다시 쓰는 것

// immutable by default
//mut

fn double(input: i32) -> i32{
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    
    //mutable variable;
    let mut my_number = 10;
    my_number = 9;

    //shadowing
    let my_variable = 10;
    let my_variable = "My variable";

    println!("{}", my_variable);

    let x = 9;
    let x = double(x);
    let x = triple(x);

    println!("{}", x);
    
    let my_variable = 9;
    {
        let my_variable = "Some string";
        println!("{}",my_variable);
    }
    println!("{}", my_variable)
}
