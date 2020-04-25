pub fn run(){
    let age: u8 = 30;
    let check_id: bool = true; 
    if age >= 21 && check_id{
        println!("bartender: what would u like to drink?")
    }else if age < 21 && check_id{
        println!("bartender: go away!")
    }else {
        println!{"bartender: your id please"}
    }
    let bool_result_shorthand= if age > 1{true} else {false};
}