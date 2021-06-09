fn main() {
    println!("Hello, world!");
    another_function(5);
    println!("Now we're not!");

    let x = {
        let y = {
            let b = 1;
            b + 1
        };
        y + 1
    };
    println!("The value of x is: {}", x);

    let q = plus_one(1);
    println!("q: {}", q);

    if 2 < 3 {
        println!("2 is greater than 3");
    } else {
        println!("You'll never get here");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result from the loop is: {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The value is: {} (iter)", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let the_array = [0, 1, 2, 3, 4, 5];
    for num in the_array.iter() {
        println!("fibb({}) is: {}", num, fibb(*num))
    }
}

fn another_function(x: i32) {
    println!("The value you passed in was: {}", x);
}

fn plus_one(x: i32) -> i32 {
    // there's no semicolon because this is an expression so it returns x+1.
    x + 1
}

fn fibb(n: usize) -> usize {
    if n == 0 || n == 1 {
        1
    } else {
        fibb(n - 1) + fibb(n - 2)
    }
}
