fn main() {
    loop_range();
}

#[allow(dead_code)]
fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
}

#[allow(dead_code)]
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
#[allow(dead_code)]
fn loop_range() {
    for number in (1..15).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let x = 2..20;
    print_type_of(&x);
}
