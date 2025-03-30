fn calc(nums: [isize; 2], operator: char) -> isize {
    println!("{:?}", nums);
    if operator == '+' {
        nums[0] + nums[1]
    } else if operator == '-' {
        nums[0] - nums[1]
    } else if operator == '*' {
        nums[0] * nums[1]
    } else if operator == '/' {
        nums[0] / nums[1]
    } else  {
        nums[0]
    }
}

fn main() {
    println!("{:?}", calc([21262, 21262], '+'));
}

