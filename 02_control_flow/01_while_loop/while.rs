fn while_stmt() {
    let mut x = 1;
    
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x is {}", x);
    }
    
    let mut y = 1;
    loop {// infinite loops
        y *= 2;
        println!("y is {}", y);
        
        if y == 1<<10 {
            break;
        }
    }
}

fn main() {
    while_stmt();
}