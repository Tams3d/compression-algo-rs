use std::fmt::Write;

pub fn compress(input: &str) -> String {
    let mut chars = input.chars();
    let Some(mut prev) = chars.next() else {
        return String::new();
    };
    let mut out = String::with_capacity(input.len());
    let mut freq: usize = 1;

    for ch in chars {
        if ch == prev {
            freq += 1;
        } else {
            push_char_freq(&mut out, prev, freq);
            prev = ch;
            freq = 1;
        }
    }
    push_char_freq(&mut out, prev, freq);
    out
}

fn push_char_freq(out: &mut String, ch: char, freq: usize) {
    out.push(ch);
    let _ = write!(out, "{freq}");
}

pub fn decompress(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        let mut count: usize = 0;
        loop {
            let d = match chars.peek() {
                Some(c) => c.to_digit(10),
                None => None,
            };
            match d {
                Some(d) => {
                    count = count * 10 + d as usize;
                    chars.next();
                }
                None => break,
            }
        }
        for _ in 0..count {
            out.push(ch);
        }
    }
    out
}
