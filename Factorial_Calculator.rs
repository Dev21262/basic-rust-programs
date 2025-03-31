
fn calculate_factorial(n: usize) -> usize {
    let mut i = n;
    let mut c = n; 
    while i != 1 {
        i -= 1;
        c = c * i;
    }

    c
}

fn main() {
    println!("{:?}", calculate_factorial(5));
}