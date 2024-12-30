fn main() {
    //String
    //&str ref str "string slice";
    //owned type

    // String = Size type
    // str = dynamic type

    let my_name = "Kim"; //&str 
    println!("{}", my_name);
    let my_name = "Kim".to_string();
    println!("{}", my_name);
    let other_name = String::from("Kim");
    println!("{}", other_name);

    //growable + shrinkable
    let mut my_other_name = "Kim".to_string();
    my_other_name.push('!');

    println!("{}", my_other_name);


    //String method

    //.capacity -> 용량
    //.push
    //.push_str
    //.pop
    //with_capacity

    let mut my_name = "Kim".to_string();
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());

    my_name.push('!');
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul");
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());
    println!("My name is {}", my_name); 

    //allocation -> 배당

    let mut my_name = String::with_capacity(20); // 해당 바이트를 넘어서게 되면 용량이 2배가 됨
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());

    my_name.push('!');
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul");
    println!("Length is {} and Capacity is : {}", my_name.len(), my_name.capacity());
    println!("My name is {}", my_name); 
      

}
