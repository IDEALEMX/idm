use std::io::stdin;

pub fn ask_for_confirmation(prompt: &str, opts: Vec<&str>) -> String {
    let mut buf: String = String::default();
    println!("{prompt}");
    let _ = stdin().read_line(&mut buf);
    while !opts.contains(&buf.as_str().trim()) {
        println!("Unknown answer: {buf}");
        println!("{prompt}");
        buf = String::default();
        let _ = stdin().read_line(&mut buf);
    }
    buf
}
