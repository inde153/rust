// trait = 초능력
//This type implements (trait name)


//From, Into
//From을 사용하면 Into사용 가능
fn main() {
    let my_name = String::from("Dave MacLeod");

    let my_city:String = "Seoul".into(); //&str
    println!("{}", my_city);
 
    let my_vec = Vec::from([8,10]); //[i32; 3]

}
