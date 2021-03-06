extern crate mac_address;

use mac_address::get_mac_address;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let input = &args[1];
    get_args();
    get_mac_address2();
}

fn get_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    return args[1].to_string();
}

fn get_mac_address2() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes()); 
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e), 
    }
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