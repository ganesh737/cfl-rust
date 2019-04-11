fn if_stmt() {
    let temp = 35; // 25 deg C
    if temp > 30 {
        println!("its really hot!!!");
    }
    else if temp < 10 {
        println!("its really cold!!!");
    }
    else {
        println!("good weather to go out!!!");
    }
    
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}", day);
    println!("Today is {}", 
        if temp > 20 {"hot"} else {"cold"});
    println!("Today is {}", 
        if temp > 20 {"hot"} else {"cold"});
    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"});
}

fn main() {
    if_stmt();
}