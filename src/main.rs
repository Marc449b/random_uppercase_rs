use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use std::io::{self, Write};
use rand;

fn save_to_clipboard(text: String) {
    let mut ctx = ClipboardContext::new().expect("Error: Failed to create clipboard context");
    ctx.set_contents(text).unwrap();
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // flush() is used to make sure that the prompt is printed before the user input
    io::stdout().flush().expect("Error: Flush failed!");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Unable to read user input!");
    return input.trim().to_string();
}

fn generate(string: String) -> String {
    // Make string empty
    let mut new_string = String::new();

    // iterate over characters in string and randomly make them uppercase or lowercase
    for c in string.chars() {
        if rand::random() {
            new_string.push(c.to_uppercase().next().unwrap());
        } else {
            new_string.push(c.to_lowercase().next().unwrap());
        }
    }

    return new_string;
}

fn main() {
    let mut string = get_input("Enter a string: ");
    string = generate(string);
    
    save_to_clipboard(string.clone());

    println!("\"{}\" copied to clipboard", string);
}
