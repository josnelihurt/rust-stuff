use std::mem;
pub fn run(){
    let nums: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", nums);
    let mut nums2: [i32; 5] = [1,2,3,4,5];
    nums2[2] = 111;
    println!("{:?}", nums2);
    println!("{:?}", nums2.len());
    println!("{:?} bytes", mem::size_of_val(&nums2));

    let slice: &[i32] = &nums2;
    println!("Slice {:?} ", slice);
    let slice2: &[i32] = &nums2[0..2];
    println!("Slice {:?} ", slice2);
}