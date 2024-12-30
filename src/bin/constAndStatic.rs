// global

//const -> 무조건 타입이 필요함 
//static -> 무조건 타입이 필요함 (같은 메모리 공간을 사용한다.)


//attribute
// #[allow(non_upper_case_globals)]  소문자인 경우 upper case를 사용하라고 하는데 워닝을 없애고 싶을 때 사용
const HIGH_SCORE : i32 = 20;
static mut LOW_SCORE: i32 = 0; // unsafe

fn print_high_score(){
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    //static lifetime -> 스태틱은 처음부터 끝까지 살 수 있음
    let my_name = "Kim"; // static str
    print_high_score();

    unsafe {LOW_SCORE = 1;} // 지양

}
