use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_closure.value(intensity)
        );
    }
}

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::cmp::Eq + std::hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> &V {
        if self.value.contains_key(&arg) {
            return &self.value[&arg];
        }
        self.value.insert(arg, (self.calculation)(arg));
        self.value.get(&arg).unwrap()
    }
}
