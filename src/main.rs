use std::{thread, time};
use text_io::read;
use std::io::{self, Write};
use notify_rust::Notification;
use figlet_rs::FIGfont;

fn timer(time: u64) {
    let timer_time = time::Duration::from_secs(time);
    let now = time::Instant::now();

    thread::sleep(timer_time);

    assert!(now.elapsed() >= timer_time);

    println!("Time's up!!");

        Notification::new()
        .summary("Focus Mode Timer Done!")
        .body("Time's up! Take a break.")
        .show()
        .expect("Failed to send notification");

}

fn main() {
loop{
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Rusty Clock");

    if let Some(figure) = figure {

                    let width = figure.to_string()
            .lines()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);


                clearscreen::clear().expect("failed to clear screen");
println!("{}", "=".repeat(width));
        println!("{}", figure);
println!("{}", "=".repeat(width));
    print!("rusty_clock>");
    io::stdout().flush().unwrap();

    let input: String = read!();

    if input.is_empty() {
        continue;
    }

    let args: Vec<&str> = input.split_whitespace().collect();

    match args.get(0) {
        Some(&"timer") => {
            print!("Pick a time in minutes: ");
            let timer_input_seconds: u64 = read!();
            println!("Inputted: {}", timer_input_seconds);
            let timer_input = timer_input_seconds * 60;
            timer(timer_input);
        }



        Some(&"exit") => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Closing now.");
            break;
        }

        Some(&"help") => {
            println!("Possible commands are 'help','exit', and 'create <task>'")
        }
        _ => {
            println!("Unknown command. Available commands: create, del, exit");
        }
    }
}
}
}

