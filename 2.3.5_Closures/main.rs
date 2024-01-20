fn main() {
    let closure = |slice: &[u32]| -> (u32, u32) {
        return (slice[0], slice[slice.len() - 1]);
        
    };
  
    println!("{:?}", closure(&[1, 2, 3, 4, 5, 6]));
}

