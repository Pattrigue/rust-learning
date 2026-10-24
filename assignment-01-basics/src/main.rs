fn main() {
    println!("1 Celsius = {} Fahrenheit", celsius_to_fahrenheit(1.0));

    println!("fibonacci(100) = {}", fibonacci(10));

    print_lyrics();
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut current = 0;
    let mut prev1: u64 = 1;
    let mut prev2: u64 = 0;

    for _ in 1..n {
        current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    current
}

fn print_lyrics() {
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for (i, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", day);

        for j in (0..=i).rev() {
            println!("{}{}", verses[j], if j == 1 { " and" } else { "" });
        }

        if i != days.len() - 1 {
            println!();
        }
    }
}
