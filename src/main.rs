use std::io::{self, Write};
fn main() {
    if start_repl().is_err() {
        panic!("REPL error");
    }
}

#[allow(unreachable_code)]
fn start_repl() -> std::io::Result<()> {
    let mut buf = String::new();
    loop {
        print_prompt(&mut buf)?;
        let input = buf.trim();
        dbg!(input);
        if input == "exit" {
            std::process::exit(0);
        }
        process(input);
        buf.clear();
    }
    Ok(())
}

#[inline(always)]
fn print_prompt(buf: &mut String) -> io::Result<()> {
    // Edit this print argument to use your app's name
    print!("App > ");
    io::stdout().lock().flush()?;
    io::stdin().read_line(buf)?;
    Ok(())
}


/// Write your logic here
fn process(cmd: &str) {
    todo!();
}
