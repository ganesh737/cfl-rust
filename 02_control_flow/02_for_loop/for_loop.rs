fn for_stmt() {
    for x in 1..11 { // 1..11, 11 is not included
        println!("x is {}", x);
    }
    
    for (pos, y) in (31..41).enumerate() {
        println!("index of {} is {}", y, pos);
    }
}

fn main() {
    for_stmt()
}