/// the stack the heap and pointers;

fn main() { // &
    let my_number = 15; // This is an i32
    let single_reference: &i32 = &my_number; //  This is a &i32 reference to my_number;
    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32
}
