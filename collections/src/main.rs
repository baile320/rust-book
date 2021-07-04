fn main() {
    // VECTOR STUFF (8.1)
    let v: Vec<i32> = Vec::new();
    println!("vec: {:?}", v);
    let mut v = vec![1, 2, 3];
    println!("vec: {:?}", v);
    v.push(4);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(5) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // panics
    let does_not_exist = v.get(100); // returns None
    println!("does_not_exist: {:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];

    let mut first = &v[0];
    // v.push(6); // doesn't work since first = &v[0] is borrowed as immutable

    println!("The first element is: {}", first);

    let v = vec![100, 23, 12];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 23, 12];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // STRING STUFF (8.2)
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str does not take ownership of s2
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{the_string}", the_string = s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // HASH MAPS (8.3)
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name); // field_name has been moved by the insert because it doesnt implement Copy trait

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let blah = scores.entry(String::from("Blue")).or_insert(50); // returns 10 since entry returns 10 and or_insert does nothing when entry returns a result

    println!("{:?}", blah);
}
