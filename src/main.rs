use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::process::Command;
use std::str;

fn main() {
    let input = Command::new("uuidgen").output().unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let len_without_newline = input.stdout.len() - 2;
    let buf_with_removed_newline = &input.stdout[..len_without_newline];
    let output_str = str::from_utf8(&buf_with_removed_newline)
        .unwrap()
        .to_owned();
    ctx.set_contents(output_str).unwrap();
    println!("uuid copied to clipboard")
}
