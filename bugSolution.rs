fn main() {
    let mut x = 5;
    {
        let y = &mut x; 
        *y = 10;
    }
    {
        let z = &mut x; 
        *z = 15; 
    }
    println!("x = {}", x);
}

// While the example works correctly, it highlights that using multiple mutable references needs careful scoping to avoid potential race conditions or undefined behavior if used within a multi-threaded environment.