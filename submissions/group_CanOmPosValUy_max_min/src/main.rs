fn main() {
    let number = [1, 2, 3, 4, 5];
    
    
    println!("Array Elements");
    for i in number {
        print!("{} ", i);
    }

    println!("\nMaximum Number: {}", find_maximum(&number));
    println!("Minimum Number: {}", find_minimum(&number));
}

fn find_maximum(number: &[i32]) -> i32 {
    let mut max = number[0];

    for &num in number {
        if num > max {
            max = num;
        }
    }
    max
}

fn find_minimum(number: &[i32]) -> i32 {
    let mut min = number[0];

    for &num in number {
        if num < min {
            min = num;
        }
    }
    min
}