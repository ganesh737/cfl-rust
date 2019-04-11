fn operators() {
    // arithmetic
    // +, -, *, /, %
    // +=, -=, *=, /=, %=
    let mut a = 2+3*4;
    println!("a = {}", a);
    a = a + 1;
    println!("a = {}", a);
    
    let a_cubed = i32::pow(a, 3);
    println!("a^3 = {}", a_cubed);
    
    let b:f64 = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b = {}", b);
    println!("b^3 = {}", b_cubed);
    println!("b^PI = {}", b_to_pi);
    
    // bitwise operators --> integers only
    // | --> OR
    // & --> AND
    // ^ --> XOR
    // ! --> NOT
    // >> --> Right Shift
    // << --> Left Shift
    let c = 1 | 2;
    println!("c = {}", c);
    let two_to_pow_ten = 1 << 10;
    println!("2^10 = {}", two_to_pow_ten);
    
    // logical operators
    // <, >, <=, >=, ==
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("PI<4 = {}", pi_less_4);
}

fn main() {
    operators();
}