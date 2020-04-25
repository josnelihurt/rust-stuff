use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let cmd = args[0].clone();
    println!("args {:?}", args);
    println!("cmd {:?}", cmd);
    
}