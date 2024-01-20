// add: takes two integers and returns their sum
// print_modulo: takes two integers and prints their modulo
// slice_sum: return the sum of all elements of a slice
fn add(number1: i32, number2 : i32) -> i32 {
    let result: i32;
    result = number1 + number2;
    return result
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for number in slice {
        total = total + number
    }
    return total
}

fn print_modulo(number1: i32, number2: i32) {
    let result: i32;
    result = number1 % number2;
    println!("{}", result)
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
  }
