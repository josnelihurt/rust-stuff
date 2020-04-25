
//Traditional struct
struct Color{
    red:u8,
    green:u8,
    blue:u8,
}
//Tuple Struct
struct Color2(u8,u8,u8);
struct Person{
    first_name: String,
    last_name: String
}
impl Person{
    //Construct 
    fn new(first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self)-> String{
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, last:&str ){
        self.last_name=last.to_string();
    }
    fn to_tuple(self)->(String,String){
        (self.first_name,self.last_name)
    }
}
pub fn run(){
    let mut c = Color{
        red: 2,
        green: 4,
        blue: 0
    };
    c.blue = 200;
    println!("Traditional Struct {} {} {}", c.red, c.green, c.blue);
    let mut c = Color2(2,3,4);
    println!("TupleStruct {} {} {}", c.0, c.1, c.2);
    
    let mut p=Person::new("john", "doe");
    println!("Person  {} {}", p.first_name, p.last_name);
    println!("Person  {} ", p.full_name());

    let mut p2=Person::new("tee", "doe");
    println!("Person  {} ", p.full_name());
    p.set_last_name("poo");
    println!("Person  {} ", p.full_name());
    println!("Person Tuple {:?} ", p.to_tuple());
    
}