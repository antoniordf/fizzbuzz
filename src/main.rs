fn main() {
    fizzbuzz(301);
}

fn fizzbuzz(n: i32) -> i32 {
    let mut count = 0;
    for n in 1..n {
        match n {
            n if n % 15 == 0 => {
                println!("FizzBuzz");
                count += 1;
            }
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            _ => println!("{}", n),
        }
    }
    count
}
