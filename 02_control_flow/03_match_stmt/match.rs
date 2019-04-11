fn match_stmt() {
    let country_code = 999;
    let country = match country_code {
        91 => "India",
        44 => "UK",
        46 => "Sweden",
        49 => "Germany",
        1...999 => "Unknown",
        _  => "Invalid"
    };
    println!("country code: {};country: {}", country_code, country);
}

fn main() {
    match_stmt()
}