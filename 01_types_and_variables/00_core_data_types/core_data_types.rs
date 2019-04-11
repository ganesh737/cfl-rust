use std::mem;

fn main() {
    // let --> immutable
    // unsigned 8 bit variable
    let a:u8 = 123;
    println!("a = {}", a);
    
    // mutable int 8 bit variable
    let mut b:i8 = 45;
    println!("b = {}", b);
    b = 4;
    println!("b = {}", b);
    
    // expect it to be i32
    let mut c = 12356789;
    println!("c = {}; size = {}", c, mem::size_of_val(&c));
    c = -123;
    println!("c = {}", c);

    // So basically --> i8 u8 i16 u16 i32 u32 i64 u64
    
    let z:isize = 123; //isize/usize for memory sizes
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}; takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    // sizeof char is 4bytes to handle all unicode, something like wchar_t
    let d:char = 'x';
    println!("d = {}; size = {}", d, mem::size_of_val(&d));
    
    // double precision, 8bytes, f64
    // so f32, f64
    let e = 2.5;
    println!("e = {}; size = {}", e, mem::size_of_val(&e));
    
    // boolean
    let f = false;
    println!("f = {}; size = {}", f, mem::size_of_val(&f));
}