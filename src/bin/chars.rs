

fn main() {

    // char 4 bytes
    let first_letter = 'A'; // char 타입은 ' ' 사용
    let space = ' ';
    let other_language_char = '9';

    // ASCII
    // u8 = 255 아스키 코드는 255까지 있기 때문에 u8을 사용하면 안전하게 캐스팅 할 수 있다.
   let my_number = 'a' as u8;

   println!("Hello, world! My number is {}", my_number);

    // casting = simple type change using 'as'

    let my_number: u16= 8;
    let second_number : u8 = 10;
    let third_number = my_number + second_number as u16;


    println!("{}", third_number);

}