fn main() {
    println!("{}", nth_fibb(50));
}

fn nth_fibb(mut n: u64) -> u64{
    let mut prev1 = 1;
    let mut prev2 = 0;
    let mut temp: u64;


    while n > 0 {
        println!("{}, {}", prev1, prev2);
        temp = prev1;
        prev1 += prev2;
        prev2 = temp;

        n -= 1;
    }

    prev1
}
