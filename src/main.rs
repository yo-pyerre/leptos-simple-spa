use leptos::*;
use std::io;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <p>"Hello, world!"</p> });

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}