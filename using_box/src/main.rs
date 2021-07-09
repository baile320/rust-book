use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(item: T) -> MyBox<T> {
        MyBox(item)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("This is getting dropped! now");
    }
}

fn main() {
    let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(x, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let new_ptr = CustomSmartPointer {
        data: String::from("This will get dropped"),
    };

    // let new_ptr2 = CustomSmartPointer {
    //     data: String::from("This will get dropped"),
    // };
    // new_ptr2.drop() // this will fail... need to call drop(new_ptr2);

    // let x = box 5; // don't do this, it's experimental

    let b = Cons(4, Rc::clone(&list));
    let c = Cons(5, Rc::clone(&list));

    println!("drop list");
    drop(list);

    println!("drop b");
    drop(b);

    println!("c is still available");
    println!("{:?}", c);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
