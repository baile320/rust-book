use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // example of splitting work into multiple worker threads. summing 0 - 299
    let accumulator = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let num_ranges = vec![0..100, 100..200, 200..300];

    for num_range in num_ranges {
        let accumulator = Arc::clone(&accumulator);
        let handle = thread::spawn(move || {
            let mut sum = accumulator.lock().unwrap();
            for num in num_range {
                *sum += num
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "The sum of the first 300 nums is: {}",
        *accumulator.lock().unwrap()
    );
}
