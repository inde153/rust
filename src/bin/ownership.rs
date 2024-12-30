//OWNERSHIP -> 소유권
//& -> reference

// fn return_it() -> &String {
//     let country = &String::from("대한민국");

//     &country // return &String
// }


fn print_country(country_name:&String){
    println!("My country is {}", country_name);
}

fn main() {
    let country = String::from("대한민국");
    let ref_one = &country;
    let ref_two = &country;
    
    println!("Country is : {}", ref_one);
    
    // let my_country = return_it(); //레퍼런스가 해제되며 알 수 없음
    

    //& immutable reference / shared reference
    //& mutmutable reference / unique reference
    let mut my_number = 9;
    let num_ref = &mut my_number;

    // num_ref = 10; 불가능
    *num_ref = 10; // 가능
    println!("{}", my_number);

    let mut my_number = 9;
    let num_ref = &mut &mut my_number;

    // num_ref = 10; 불가능
    **num_ref = 10; // 가능
    println!("{}", my_number);

    //  let mut number = 10;
    //  let number_ref = &number; 러스트에서는 불변과
    //  let number_change = &mut number; 가변을 같이  사용할 수 없음

    // *number_change += 10;

    // println!("{}",number_ref);

    // 이렇게 하면 2~3번 실행 가능
    let country = "대한민국".to_string();
    print_country(&country);
    print_country(&country);
}
