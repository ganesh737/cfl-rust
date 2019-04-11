use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 1.0, y:1.0}
}

pub fn sh() {
    // stack allocated i.e. p1 is an object of Point
    let p1:Point = origin();
    // heap allocated i.e. p2 os a pointer
    let p2 = Box::new(origin());
    
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    
    // access data in p2
    let p3 = *p2;
    println!("x:{},y:{}", p3.x, p3.y);
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
}