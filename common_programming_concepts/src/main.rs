fn main() {
    variables_and_mutability();
    data_types();

    sum(5, 3);

    max(5, 3);

    looping();
}

fn variables_and_mutability(){
    let x = 5;
    println!("The value of x is: {}", x);
    
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 5;
    println!("The value of y is: {}", y);

    const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60;
}

fn data_types(){
    //parse requires a type to be specified.
    let guess: u32 = "42".parse().expect("Not a number!");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("0: {}, 3: {}", a[0], a[3]);
    for e in a {
        print!("{}, ", e);
    }
    println!();

    let a = [3; 5];
    for e in a {
        print!("{}, ", e);
    }
    println!();
}

fn sum(a: i32, b: i32) -> i32 {
    if a <= 0 || b <= 0 {
        return 0;
    }

    a + b
}

fn max(a: i32, b: i32) -> i32{
    if a > b {a} else {b}
}

fn looping(){
    let mut count = 0;

    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End Count = {}", count);


    let mut counter = 0;

    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };


    while result != 0 {
        println!("{}!", result);
        result -= 1;
    }
    println!("LIFTOFF!!!");


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
