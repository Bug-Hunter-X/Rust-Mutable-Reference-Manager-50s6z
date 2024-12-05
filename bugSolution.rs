fn main() {
    let mut x = 5;
    { // using block to limit the scope of the borrow
        let y = &mut x; 
        *y = 6;
        println!("x = {}", x);
    }
    { // a new borrow for next operation
        let z = &mut x;
        *z = 7;
        println!("x = {}", x);
    }

} 