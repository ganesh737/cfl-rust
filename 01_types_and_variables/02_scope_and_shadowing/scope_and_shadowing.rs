fn scope_and_shadowing() {
    let a = 123;
    {
        // shadows the a
        let a = 321;
        println!("a = {}", a);
    }
    println!("a = {}", a);
}

fn main() {
    scope_and_shadowing()
}