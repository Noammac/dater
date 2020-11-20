extern crate chrono;

use chrono::prelude::{Date, Local, Datelike};

fn main() {
    let now: Date<Local> = Local::now().date();

    let ord = now.ordinal();
    let year = now.year();

    println!("Today is the {} day of the year {}, or {:#X} in hex.", ordinate(ord), year, ord);

    if ord == 256 {
        println!("Happy day of the Programmer!");
    }
    if (now.month() == 11) & (now.day() == 20) {
        println!("Happy birthday! You are now {} years old!", year - 2002);
    }
}

fn ordinate(num: u32) -> String {
    let suff = match num % 100 {
        _a @ 4..=20 => ("th"),
        dec => { match dec % 10 { 1 => ("st"), 2 => ("nd"), 3 => ("rd"), _ => "th" } }
    };
    format!("{}{}", num, suff)
}
