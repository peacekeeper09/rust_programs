use rand::{Rng, thread_rng};
use rand::rngs::OsRng;
use rand::distributions::Distribution;

fn main() {
    let password = generate_password(16, true, true, true, true);
    println!("Your random password is: {}", password);
}

fn generate_password(length: usize, uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> String {
    let mut rng = OsRng;
    let mut charset = String::new();
    if uppercase {
        charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if lowercase {
        charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if numbers {
        charset += "0123456789";
    }
    if symbols {
        charset += "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    }
    if charset.is_empty() {
        panic!("At least one character set must be enabled.");
    }
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    password
}
