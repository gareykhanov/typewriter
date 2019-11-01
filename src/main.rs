use std::io::{BufRead};
use std::fs::{File};

struct ScreenInfo {
    width: usize,
    height: usize
}

fn main() {
    let screen_info = ScreenInfo { width: 130, height: 24 };
    let chars = read_file("test/test.txt").unwrap();
    print(&chars, &screen_info);
    //for c in chars.iter() {
    //    let v = vec![c];
    //    println!("char: {}, len: {}", c, v.len());
    //}
}

fn read_file(path: &str) -> Result<Vec<char>, String> {
    let file_str = std::fs::read_to_string(path).map_err(|e| format!("read file error {}", e))?;
    Ok(file_str.chars().collect())
}

fn print(print_buf: &Vec<char>, screen_info: &ScreenInfo) {

    let mut line = 0;
    let mut word = String::new();
    let mut word_len = 0;
    for c in print_buf.iter() {
        let c = *c;
        if (c != '\n') && (c != ' ') {
            word.push(c);
            word_len += 1;
        } else if word_len > 0 { 
            let nl_c = c == '\n';
            let c_len = if nl_c { 0 } else { 1 };
            let line_with_word = word_len + line;
            let (_line, print_c, new_line) = if line_with_word > screen_info.width {
                (word_len + c_len, true, true)   
            } else if line_with_word + c_len < screen_info.width {
                (if nl_c { 0 } else { line_with_word + c_len }, true, false)
            } else if line_with_word + c_len == screen_info.width {
                (0, true, false)
            } else {
                (0, false, false)
            };
            
            line = _line;
            if new_line {
                println!();
            }
            print!("{}", word); 
            if print_c {
                print!("{}", c);
            }
            word.clear();
            word_len = 0;
        } else if c == '\n' {
            println!();
        }
    }
}

