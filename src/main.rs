use rand::Rng;

struct GenSettings {
    numbers: bool,
    strings: bool,
    symbols: bool,
}

fn main() {
    struct arguments {
        length: usize,
        numbers: bool,
        strings: bool,
        symbols: bool,
    }
    let argstoparse = std::env::args().collect::<Vec<String>>();
    let mut args = arguments {
        length: 0,
        numbers: false,
        strings: false,
        symbols: false,
    };
    args.length = argstoparse[1].parse::<usize>().unwrap();
    args.numbers = argstoparse[2].parse::<bool>().unwrap();
    args.strings = argstoparse[3].parse::<bool>().unwrap();
    args.symbols = argstoparse[4].parse::<bool>().unwrap();
    gen(args.length, args.numbers, args.strings, args.symbols);
}

fn gen(char: usize, abc: bool, num: bool, sym: bool) {
    // setup struct from args
    let settings = GenSettings {
        numbers: num,
        strings: abc,
        symbols: sym,
    };
    let numrand = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    // all capital and lowercase letters
    let charrand = vec![
        "a", "A", "b", "B", "c", "C", "d", "D", "e", "E", "f", "F", "g", "G", "h", "H", "i", "I",
        "j", "J", "k", "K", "l", "L", "m", "M", "o", "O", "p", "P", "q", "Q", "r", "R", "s", "S",
        "t", "T", "u", "U", "v", "V", "w", "W", "x", "X", "y", "Y", "z", "Z",
    ];
    let symbols = vec![
        "_", "&", "$", "%", "*", "!", "?", "~", "^", "=", "+", "-", ";", ">", "<", "]", "[",
    ];
    let mut password = Vec::new();
    let mut finalpass = String::new();
    let chartype = vec![&numrand, &charrand, &symbols];
    let mut pick: &Vec<&str>;
    match settings {
        GenSettings {
            numbers: true,
            strings: true,
            symbols: true,
        } => {
            let chartype = vec![&numrand, &charrand, &symbols];
            for _i in 0..char {
                // pick a random character type
                pick = chartype[rand::thread_rng().gen_range(0..3)];
                // push a random character from the previously chosen type
                password.push(pick[rand::thread_rng().gen_range(0..pick.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: true,
            strings: false,
            symbols: false,
        } => {
            for _i in 0..char {
                // push a random character from the previously chosen type
                password.push(numrand[rand::thread_rng().gen_range(0..numrand.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: false,
            strings: true,
            symbols: false,
        } => {
            for _i in 0..char {
                // push a random character from the previously chosen type
                password.push(charrand[rand::thread_rng().gen_range(0..charrand.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: false,
            strings: false,
            symbols: true,
        } => {
            for _i in 0..char {
                // push a random character from the previously chosen type
                password.push(symbols[rand::thread_rng().gen_range(0..symbols.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: true,
            strings: true,
            symbols: false,
        } => {
            let chartype = vec![&numrand, &charrand];
            for _i in 0..char {
                // pick a random character type
                pick = chartype[rand::thread_rng().gen_range(0..2)];
                // push a random character from the previously chosen type
                password.push(pick[rand::thread_rng().gen_range(0..pick.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: true,
            strings: false,
            symbols: true,
        } => {
            let chartype = vec![&numrand, &symbols];
            for _i in 0..char {
                // pick a random character type
                pick = chartype[rand::thread_rng().gen_range(0..2)];
                // push a random character from the previously chosen type
                password.push(pick[rand::thread_rng().gen_range(0..pick.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        GenSettings {
            numbers: false,
            strings: true,
            symbols: true,
        } => {
            let chartype = vec![&charrand, &symbols];
            for _i in 0..char {
                // pick a random character type
                pick = chartype[rand::thread_rng().gen_range(0..2)];
                // push a random character from the previously chosen type
                password.push(pick[rand::thread_rng().gen_range(0..pick.len())]);
            }
            // build string
            for i in password {
                finalpass.push_str(i);
            }
            println!("{}", finalpass);
        }
        _ => {
            // generate nothing
            println!("Invalid settings");
        }
    }
}