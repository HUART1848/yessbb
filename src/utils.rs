use std::io::Write;

pub fn flushed_print(text: &str) {
    print!("{}", text);
    std::io::stdout().flush().expect("print error");
}
