use rand::Rng;

fn main() {
    println!("Hello, world!");
}

fn gen_rand_str() {
    let mut alphabet_vec: Vec<char> = vec![];

    for _num in 1..6 {
        // Unicodeのコードポイントのために、97から122の乱数を発生させています
        let rand_num = rand::thread_rng().gen_range(97..123);
        if let Some(rand_num) = std::char::from_u32(rand_num) {
            alphabet_vec.push(rand_num);
        }
    }

    let alphabet_str: String = alphabet_vec.iter().collect::<String>();
    println!("{}", alphabet_str);
}

// #[cfg(test)]
// mod tests {
//     fn create_password_ok() {
//         let pass_length = 5;

//     }
// };