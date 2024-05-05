use std::io;

fn find_nemo(words: &str) -> String {

    let mut i : u32 = 0;
    for word in words.split_whitespace() {
        i += 1;
        if word == "Nemo" {
            return format!("I found Nemo at {}", i);
        }
    }

    return String::from("I can't find Nemo :(")
    // iterate over tokens until first nemo is found
}

fn main() {
    loop {
        println!("Enter a set of space-separated words:");

        let mut words = String::new();

        // take input
        io::stdin()
            .read_line(&mut words)
            .expect("Failed to read sentence");

        println!("{0}", find_nemo(& words));
    }
}

// Ref: https://doc.rust-lang.org/rust-by-example/std/str.html