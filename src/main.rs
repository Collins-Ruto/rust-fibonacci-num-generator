use std::io;

fn main() {
    println!("Hello, and wlcome to the Fibonacci number generator");

    let mut x = 0;
    let mut y = 1;
    let mut count = 0;

    let mut nth = String::new();

    println!("What nth term are you interested in ?");

    io::stdin()
        .read_line(&mut nth)
        .expect("You better input a number");

    let nth:u32 = nth.trim().parse()
                .expect("well it wasn't a number was it ?");
    
    println!("Seems you are looking for {}nth term", nth);

    while count < nth {
        let z = x;
         x = if count < 1 {0} else {y};
        y =  y + z ;

        // println!("x is currently at {} and y is {}", x, y);
        count += 1
    }

    println!("Your {}nth number should be {}",nth, y)
}
