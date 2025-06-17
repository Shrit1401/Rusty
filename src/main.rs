use colored::*;
use std::io;
use std::io::Write;
use whoami;

mod ai_chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcome to Rusty!".green().bold().to_string());
    println!(
        "{}",
        "So Tell me what you wanna do today? (Type 'exit' to quit)".yellow()
    );

    let username = whoami::username();

    loop {
        let mut input = String::new();
        print!("{}: ", username.green());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("{}", "Goodbye!".green());
            break;
        }

        let response = ai_chat::ai_response(input).await?;
        ai_chat::display_ai_response(&response);
        println!(
            "{}",
            "\nWhat else would you like to know? (Type 'exit' to quit)".yellow()
        );
    }

    Ok(())
}
