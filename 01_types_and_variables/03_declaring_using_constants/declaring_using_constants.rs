
// no fixed address for below variable
// treated like #define
const X_CONST:u8 = 42;

static mut Y_STATIC:i32 = 123;

fn func() {
    println!("func");
    println!("x = {}", X_CONST);
    unsafe {
        Y_STATIC = 13;
        println!("y = {}", Y_STATIC);
    }
}

fn main() {
    println!("main");
    unsafe {
        Y_STATIC = 12;
        println!("y = {}", Y_STATIC);
    }
    func();
}