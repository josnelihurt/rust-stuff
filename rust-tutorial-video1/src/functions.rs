pub fn run(){
    greeting("hello", "josh");
    let get_sum = add(4,3);
    println!("{}", get_sum);

    //clousure 
    let n3: i32 = 3;
    let add_num = |n1: i32, n2: i32| n1+n2+n3;
    println!("Clouser result {}", add_num(9,9));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}! ", greet, name);
}
 
fn add(n1: i32, n2: i32) -> i32{
    n1+n2
}