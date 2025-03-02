use std::io;

fn increase(counterparam: &mut i32) {
    *counterparam += 1;
}

fn decrease(counterparam_d: &mut i32) {
    *counterparam_d -= 1;
}
fn main() {

    let mut counter= 0;

    println!("-----------------------------");
    println!("Functions - Ben Archard");
    println!("Date Created: 2nd March 2025");
    println!("-----------------------------");
    
    loop {
        let mut choice = String::new();
        println!("Counter is at {counter}");

        println!("Options");
        println!("1. Increase Counter");
        println!("2. Decrease Counter");
        println!("3. Exit");
        println!("Choice: ");

        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        if choice == 1 {
            increase(&mut counter);
        } else if choice == 2 {
            decrease(&mut counter);
        } else {
            break;
        }
    }
}

