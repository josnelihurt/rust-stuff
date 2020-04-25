pub fn run(){
    let mut count = 0;

    //infinite loop
    loop{
        count += 1;
        println!("x:{}",count );
        if count > 20{
            break;
        }
    }

    //FizzBuzz
    count = 0;
    while count <= 100{
        if count % 15 == 0{
            println!("FizzBuzz");
        } else if count %3 == 0{
            println!("Fizz");
        } else if count%5 == 0{
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
    for x in 0..100{
        println!("{}", x);
    }
}