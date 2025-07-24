use std::io;

fn main() {
    let mut tries = 0;

    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        tries += 1;

        if input == "The letter e" {
            println!("Number of trials: {}", tries);
            break;
        } else {
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        }
    }

}
