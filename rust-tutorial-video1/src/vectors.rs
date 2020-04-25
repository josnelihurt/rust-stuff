pub fn run(){
    let mut nums : Vec<i32>  = vec![1,2,3,4,5];
    println!("{:?}", nums);
    println!("Vector len {}", nums.len());

    
    for i in nums.iter_mut(){
        *i *= 2;
    }    
    nums.push(2);
    nums.pop();
    println!("{:?}", nums);
}