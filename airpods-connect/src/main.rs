fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let input = &args[1];

    println!("{}", get_mac_adress());
}

fn get_mac_adress() -> String {
    let args: Vec<String> = std::env::args().collect();
    return args[1].to_string();
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_macadress_ok() {
        let expcted_macadress = "hogehogehugahuga";
        let input_macadress = "hogehogehugahuga";
        assert_eq!(expcted_macadress, input_macadress);
    }
}