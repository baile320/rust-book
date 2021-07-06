#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // let mut counter = Counter::new();
    // assert_eq!(counter.count, 0);
    // assert_eq!(counter.next(), Some(1));
    // assert_eq!(counter.next(), Some(2));
    // assert_eq!(counter.next(), Some(3));
    // assert_eq!(counter.next(), Some(4));
    // assert_eq!(counter.next(), Some(5));
    // assert_eq!(counter.next(), None);

    let counter = Counter::new();
    println!("{}", counter.map(|x| { x * 2 }).sum::<u32>());

    let stuff: Vec<(u32, u32)> = Counter::new().zip(Counter::new().skip(1)).collect();
    // .map(|(a, b)| a * b)
    // .filter(|x| x % 3 == 0)

    println!("{:?}", stuff);
}
