use std::panic;

pub fn run(){
    let no_mutable = "hi there";
    let mut string_can_increase = String::from("hi there");
    println!("len no mut {}",no_mutable.len());
    string_can_increase.push_str(" X Y Z");
    println!("len mut {}", string_can_increase.len());
    for item in string_can_increase.split_whitespace(){
        println!("item {}", item);
    }
    let mut string_cap = String::with_capacity(2);
    string_cap.push('A');
    string_cap.push('B');
    //assert
    println!("{}",string_cap);
    let result = panic::catch_unwind(|| {
        assert_eq!(3, string_cap.len());
    });
    assert!(result.is_err());
    //assert_eq!(3, string_cap.len());
}