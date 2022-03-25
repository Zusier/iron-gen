use rand::{thread_rng, prelude::IteratorRandom};

fn main() {
    struct Arguments {
        numbers: bool,
        strings: bool,
        symbols: bool,
    }
    let argstoparse = std::env::args().collect::<Vec<String>>();
    let mut args = Arguments {
        numbers: false,
        strings: false,
        symbols: false,
    };
    let length = argstoparse[1].parse::<usize>().unwrap();
    args.numbers = argstoparse[2].parse::<bool>().unwrap();
    args.strings = argstoparse[3].parse::<bool>().unwrap();
    args.symbols = argstoparse[4].parse::<bool>().unwrap();
    let mut numrand = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    // all capital and lowercase letters
    let mut charrand = vec![
        "a", "A", "b", "B", "c", "C", "d", "D", "e", "E", "f", "F", "g", "G", "h", "H", "i", "I",
        "j", "J", "k", "K", "l", "L", "m", "M", "o", "O", "p", "P", "q", "Q", "r", "R", "s", "S",
        "t", "T", "u", "U", "v", "V", "w", "W", "x", "X", "y", "Y", "z", "Z",
    ];
    let mut symbols = vec![
        "_", "&", "$", "%", "*", "!", "?", "~", "^", "=", "+", "-", ";", ">", "<", "]", "[",
    ];

    // List of characters to be used in password
    let mut charlist: Vec<&str> = Vec::new();

    // Append to the character list according to arguments
    if args.numbers { charlist.append(&mut numrand); }
    if args.strings { charlist.append(&mut charrand); }
    if args.symbols { charlist.append(&mut symbols); }

    // Generate the password
    let password: Vec<&str> = charlist.iter().copied().choose_multiple(&mut thread_rng(), length);
    println!("{}", password.join(""));
}