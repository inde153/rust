//Collection types
//Array
//buffer 만들 때 많이 사용

fn main() {
    let arr = [1,2,3,4];

    let arr2 = ["One","Two"];//[&str;2]
    let arr3 = ["One","Two","Five"];//arr2와 다른 타입임 [&str;3] 
    let arr4: [&str; 2] = ["One","Twoo"];//[&str;2]

    println!("Is array the same as {} arr2?", arr2 == arr4);

    let arr5 = [0; 640]; //0으로 640번
    println!("{:?}", arr5);
    

    //잘 변하지 않는 것에 사용함
    let arr6 = ["1월", "2월"]; //indexing
    println!("{:?}", arr6[0]); //first
    println!("{:?}", arr6.get(1)); ////Some None Option enum;

    
    //dynamically sized type
    let seasons = ["봄","여름","가을","겨울"];
    // println!("{:?}", seasons[0..2]); // 에러 얼마나 긴지 알 수 없기 때문에
    
    println!("{:?}", &seasons[0..2]); // up to and including
    println!("{:?}", &seasons[..]);
    println!("{:?}", &seasons[3..]);
    println!("{:?}", &seasons[..=2]);


    //vecs
    //Vec<String> Vec<u8>
    //T = some type
    //generic
    let my_string = String::new();
    // let my_vec = Vec::new(); //타입을 알 수 없어서 컴파일 에러가 발생함
    let mut my_vec: Vec<String> = Vec::new();
    let name1 = String::from("windy");
    let name2 = String::from("Gomesy");

    println!("Space for my_vec {} ", my_vec.capacity());
    my_vec.push(name1);
    println!("Space for my_vec {} ", my_vec.capacity());
    my_vec.push(name2);
    println!("Space for my_vec {} ", my_vec.capacity());

    println!("My cats are {:?}", my_vec);

    let my_vec2: Vec<&String> = vec![&my_vec[0], &my_vec[1]];
    println!("My cats are {:?}", my_vec2);

}
