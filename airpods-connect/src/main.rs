fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let input = &args[1];

    // let success_message = "connected!";
    // let _error_message = "faild connect";
    println!("{}", get_mac_adress());
}

fn get_mac_adress() -> String {
    let args: Vec<String> = std::env::args().collect();
    return args[1].to_string();
}