//create smart pointers
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use crate::List::{Cons, Nil};

    //call the enum
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );
    println!("the list is : {:#?}", list);
}
