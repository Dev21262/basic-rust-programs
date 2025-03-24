const FIRST_NUMBERS: [u64; 2] = [4, 5];

fn nth(n: u64) {
    //0, 1
    //0 + 1 * 1, - 3rd number
    //0 + 1 * (n - 2) - 4th number
    let mut fb: u64 = 21262;
    if n == 1 {
        fb = FIRST_NUMBERS[0];
    } else if n == 2 {
        fb = FIRST_NUMBERS[1];
    } else {
        fb = (FIRST_NUMBERS[0] + FIRST_NUMBERS[1]) * (n - 2);   
    }
    println!("The nth fibonacci number is {fb}");
}

fn nth_fibonacci(n: u64) -> u64 {
    if n == 1 {
        return FIRST_NUMBERS[0];
    } else if n == 2 {
        return FIRST_NUMBERS[1];
    }

    let mut a = 0;
    let mut b = 1;
    let mut fib = 0;

    for _ in 3..=n {
        fib = a + b;
        a = b;
        b = fib;
    }

    fib
}


fn main() {
    nth(4)
}