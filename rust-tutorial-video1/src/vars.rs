pub fn run(){
    let name = "Josh";
    let mut age = 35;
    age+=1;
    println!("My name is {} and I am {}", name, age);
    // const 
    const ID: i32 = 001;
    println!("ID: {}",ID);
    // multiple assigments 
    let(my_name,my_age) = ("josh",35);
    println!("{name} is {age}",name=my_name,age=my_age);
     
}