fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let reference_to_nothing = dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

// this isn't allowed because it creates a dangling pointer
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn dangle() -> String {
    String::from("hello")
}
