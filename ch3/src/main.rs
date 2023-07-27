use std::mem::size_of_val;

const fn compute(x: u8) -> u8 {
    if x < 200 {
        let y = x + 3;
        return y + 50;
    }
    return x + 100;
}

fn fib(n: u32) -> u128 {
    let mut fib_nums = vec![0; n as usize];
    fib_nums[1] = 1;

    for i in 2..=(n-1) as usize {
        fib_nums[i] = fib_nums[i - 1] + fib_nums[i - 2];
    }

    fib_nums[(n - 1) as usize]
}

fn twelve_days_of_christmas() {
    let days_of_christmas = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings (five golden rings)",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days_of_christmas[day]);
        for j in (0..day+1).rev() {
            if j == 0 && day != 0 {
                println!("And {}", gifts[j]);
            }
            else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
    println!("And {}", gifts[0]);

}

const TIME: u8 = compute(10);

fn main() {
    let mut x = {
        let y = 4.;
        y + 1.0
    };
    let y = compute;
    println!("{}", y(10));
    x /= 2.5;
    let tup = (3, 0.2, vec![2, 3, 5, 2]);
    println!("{} {}", TIME, x);
    println!("{:?}", TIME.wrapping_add(255));
    println!("{}", size_of_val(&tup.2));
    let y = 2;
    let z = match y {
        y if y >= 10 && y <= 20 => 1,
        y if y < 10 => 2,
        _ => 3
    };

    let _z = {
        if y >= 10 && y < 10 {1}
        else if y < 10 {2}
        else {3}
    };
    println!("x = {}, z = {}", x, z);

    for i in (1..=4).step_by(2) {
        println!("{i}");
    }
    println!("{}", fib(11));
    twelve_days_of_christmas();
}
