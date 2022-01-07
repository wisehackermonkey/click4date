extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use chrono::{Datelike, Utc};

//copy to clipboard a string passed as argument
fn clipboard_paste(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
     ctx.set_contents(text.to_owned()).unwrap();
}

// formats current date into the following format: yyyymmdd
fn formate_date() -> String{
    let now = Utc::now();
    // for people new to rust (aka me) `0>width$` is a bit of a hack to get the right width
    // and :? allows for the use of a variable in the format string
    return format!("{:?}{:0>width$}{:0>width$}", now.year(), now.month(), now.day(),width=2);
}

fn main(){
    let current_date = formate_date();
    clipboard_paste(&current_date);
}