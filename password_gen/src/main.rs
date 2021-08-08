use arboard::Clipboard;
use structopt::StructOpt;
use rand::{Rng, thread_rng};

#[derive(StructOpt)]
struct Argument {
    number_of_digits: u64,
}
fn main() {
    let args = Argument::from_args();

    let mut alphanumerics = (0..26).map(|x| (x + b'a') as char)
        .collect::<Vec<_>>();
    let mut numbers = (0..10).map(|x| (x + b'0') as char)
        .collect::<Vec<_>>();
    alphanumerics.append(&mut numbers);

    let mut result = String::new();
    let mut rng = thread_rng();
    for i in 0..args.number_of_digits {
        let upper_bound = if i == 0 { 10 } else {36};
        result = result + &alphanumerics[rng.gen_range(0..upper_bound)].to_string();
    }
    println!("{}", result);
    add_clipboard(result);

}

// fn gen_rand_str() {
//     let mut alphabet_vec: Vec<char> = vec![];

//     for _num in 1..6 {
//         // Unicodeのコードポイントのために、97から122の乱数を発生させています
//         let rand_num = rand::thread_rng().gen_range(97..123);
//         if let Some(rand_num) = std::char::from_u32(rand_num) {
//             alphabet_vec.push(rand_num);
//         }
//     }

//     let alphabet_str: String = alphabet_vec.iter().collect::<String>();
//     println!("{}", alphabet_str);
// }

// fn copy_password(password: String) {
//     println!("{}", "copy password!!");
//     println!("{}", password);
// }
    fn add_clipboard(cory_word: String) {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(cory_word.into()).unwrap();
    }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn clipboard_ok () {
//         let pat = "hello world";
//         add_clipboard(pat);
//     }

// }