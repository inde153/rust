//uninitialized variable
//control flow


fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;

        if counter % 50  == 0 {
            break;
        }

    }

    counter
}
fn main() {

    //uninitialized variable 불가능
    //possibly uninitialized = maybe doesn't have a value yet
    let my_number:u8;

    {
        my_number = 9;
    }

    println!("{}", my_number);


    let my_number = {
        //복잡한 것들
        let x = 9;

            x + 9
    };

    println!("{}", my_number);

    let my_number:i32;
        
    {
        //복잡한 것들
        let x = loop_then_return(43);
        my_number = x;
    }; 

    println!("{}", my_number);
}
