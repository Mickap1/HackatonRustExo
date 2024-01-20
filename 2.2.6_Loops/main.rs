fn print_slices_element(slice: &[String]) {
    for strings in slice {
        println!("{}", strings)
    }
}

fn main() {
  print_slices_element(&["Thomas".to_string(), "Nassim".to_string(), "Guillaume".to_string()]);
}
