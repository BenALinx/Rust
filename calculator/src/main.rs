use std::io;

fn main() {

    let mut choice = String::new();
    let mut first = String::new();
    let mut second = String::new();
    let mut result = String::new();

    println!("----------------------------");
    println!("Calculator - Ben Archard");
    println!("Date Created: 1st March 2025");
    println!("----------------------------");
    println!("Options: ");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Division");
    println!("4. Multiplication");
    println!("----------------------------");
    println!("Choice: ");
    io::stdin().read_line(&mut choice).unwrap();

    // Convert to integer
    let choice: i32 = choice.trim().parse().unwrap();

    if choice == 1 {
        let mut first = String::new();
        let mut second = String::new();
        let mut result = String::new();

        println!("First number: ");
        io::stdin().read_line(&mut first).unwrap();

        println!("Second number: ");
        io::stdin().read_line(&mut second).unwrap();

        let first: i32 = first.trim().parse().unwrap();
        let second: i32 = second.trim().parse().unwrap();

        let result = first + second;
        println!("The result of {first} + {second} is {result}");

    } else if choice == 2 {
        let mut choice = String::new();
        let mut first = String::new();
        let mut second = String::new();
        let mut result = String::new();

        println!("First number: ");
        io::stdin().read_line(&mut first).unwrap();

        println!("Second number: ");
        io::stdin().read_line(&mut second).unwrap();

        println!("Opions");
        println!("{first} - {second}");
        println!("{second} - {first}");
        println!("Choice: ");

        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        let first: i32 = first.trim().parse().unwrap();
        let second: i32 = second.trim().parse().unwrap();

        if choice == 1 {
            let result = first - second;

            println!("The result of {first} - {second} is {result}");
        } else {
            let result = second - first;

            println!("The result of {second} - {first} is {result}");
        }
    } else if choice == 3 {
        let mut choice = String::new();
        let mut first = String::new();
        let mut second = String::new();
        let mut result = String::new();

        println!("First number: ");
        std::io::stdin().read_line(&mut first).unwrap();

        println!("Second number: ");
        io::stdin().read_line(&mut second).unwrap();

        println!("Opions");
        println!("{first} / {second}");
        println!("{second} / {first}");
        println!("Choice: ");

        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        let first: i32 = first.trim().parse().unwrap();
        let second: i32 = second.trim().parse().unwrap();

        if choice == 1 {
            let result = first / second;

            println!("The result of {first} / {second} is {result}");
        } else {
            let result = second / first;

            println!("The result of {second} / {first} is {result}");
        }
    
    } else if choice == 4 {
        let mut first = String::new();
        let mut second = String::new();
        let mut result = String::new();

        println!("First number: ");
        io::stdin().read_line(&mut first).unwrap();
        
        println!("Second number: ");
        io::stdin().read_line(&mut second).unwrap();

        let first: i32 = first.trim().parse().unwrap();
        let second: i32 = second.trim().parse().unwrap();

        let result = first * second;

        println!("The result of {first} * {second} is {result}");
    }
}
