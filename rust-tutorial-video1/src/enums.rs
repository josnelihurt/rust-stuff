enum Movement{
    //Variants
    Up,
    Down,
    Left,
    Right
}
fn move_avatar(m: Movement){
    //Perform action on input
    match m {
        Movement::Down => println!("Go Down"),
        Movement::Up => println!("Go Up"),
        Movement::Left => println!("Go Left"),
        Movement::Right => println!("Go Right"),
    }
}

pub fn run(){
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}