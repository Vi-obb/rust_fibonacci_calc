use std::io;


fn main() {
    println!("Which fibonacci number do you want to know?");

        let nth_fibonacci: u128 = loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u128>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input. Please type a positive number."),
            }
        };

    fib(nth_fibonacci)

}

fn fib(nth_fibonacci:u128) {
    let mut first_fib:u128 = 0;
    let mut second_fib:u128 = 1;

    let mut fib_value = 0;

    if nth_fibonacci == 0 {
        fib_value = first_fib
    } else if nth_fibonacci == 1 {
        fib_value = second_fib
    } else {
        for _ in 2..=nth_fibonacci {
            fib_value = first_fib + second_fib;

            first_fib = second_fib;

            second_fib = fib_value;
        }
    }

    println!("Your fibonacci number is {fib_value}");
}
