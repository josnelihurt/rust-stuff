use std::error::Error;
use std::rc::Rc;
use std::sync::Mutex;
pub fn run() {
    println!("******** ******** ******** ********");
    let mut e = Element::new();
    let player = Player::new();
    e.components.push(player);
    e.call_components();
}
struct Element {
    components: Vec<Rc<Mutex<dyn Component>>>,
}
impl Element {
    fn new() -> Self {
        Element {
            components: Vec::new(),
        }
    }
    fn print_name(&self) {
        println!("element here!");
    }
    fn call_components(&self) {
        for item in self.components.iter() {
            item.lock().unwrap().print_name();
            item.lock().unwrap().call_parent(&self);
        }
    }
}

trait Component {
    fn print_name(&mut self);
    fn call_parent(&mut self, parent: &Element);
}

struct Player {}
impl Player {
    fn new() -> Rc<Mutex<Self>> {
        Rc::new(Mutex::new(Player {}))
    }
}
impl Component for Player {
    fn print_name(&mut self) {
        println!("component a here");
    }
    fn call_parent(&mut self, parent: &Element) {
        parent.print_name();
    }
}
