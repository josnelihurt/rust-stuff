pub fn run(){
    // i32
    let x = 1;
    let xy: i8 = 2;
    println!("max i32:{}",std::i32::MAX);
    
    let is_active = true;
    let is_greater = 1>-1;
    println!("{:?}",(x,xy,is_active,is_greater));

    let ch = '\u{1F600}';
    println!("{:?}",(x,xy,is_active,is_greater,ch));    
}