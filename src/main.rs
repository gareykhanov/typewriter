use std::io::{BufRead};
use std::fs::{File};

struct ScreenInfo {
    width: usize,
    height: usize
}

#[derive(Debug)]
struct ScreenLine {
    length: usize,
    row: usize,
    nextLineCh: char
}
impl ScreenLine {
    fn new (row: usize, length: usize, nextLineCh: char) -> ScreenLine {
        ScreenLine{ length, row, nextLineCh }
    }
}


fn main() {
    let screen_info = ScreenInfo { width: 130, height: 24 };
    let chars = read_file("test/test.txt").unwrap();
    let lines = print(&chars, &screen_info);
    println!();
    for l in lines.iter() { 
        println!("{:?}", l);
    }
    
}

fn read_file(path: &str) -> Result<Vec<char>, String> {
    let file_str = std::fs::read_to_string(path).map_err(|e| format!("read file error {}", e))?;
    Ok(file_str.chars().collect())
}

fn print(print_buf: &Vec<char>, screen_info: &ScreenInfo) -> Vec<ScreenLine> {
    
    //word
    let mut w_i = None;
    let mut w_len = 0;
    //word delimeter
    let mut del = '\0';
    let mut del_len = 0;
    
    let mut line_len = 0;

    let mut lines = Vec::new();
    let mut row = 0;
    
    for (i, &c) in print_buf.iter().enumerate() {

        if c != '\n' && c != ' ' { 
            if w_i == None {
                w_i = Some(i);
            }
            w_len += 1;
        } else {
            if w_len > 0 {
                let line_with_w = line_len + w_len + del_len;
                let (on_new_line) = if del == '\n' {
                    (true)
                } else if line_with_w > screen_info.width {
                    (true)
                } else if line_with_w <= screen_info.width {
                    (false)
                } else {
                    unreachable!();
                };
                if on_new_line {
                    lines.push(ScreenLine::new(row, line_len, del));
                    row += 1;
                    line_len = w_len;
                    print!("\n");
                } else {
                    line_len += w_len + del_len;
                    if del != '\0' {
                        print!("{}", del);
                    }
                }
                for c in print_buf[w_i.unwrap()..w_i.unwrap() + w_len].iter() {
                    print!("{}", c);
                }
                if i == print_buf.len() - 1 {
                    lines.push(ScreenLine::new(row, line_len, '\0'));
                }
                //print!("{},{}", w_len, line_len);
                w_i = None;
                w_len = 0;
                del = c;
                del_len = if c == '\n' { 0 } else { 1 };
            }
        }
        
    }
    
    lines 
}



