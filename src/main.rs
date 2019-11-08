use std::fs::File;
use std::io::BufRead;

struct ScreenInfo {
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct PrintedLine {
    length: usize,
    row: usize,
    next_line_char: char,
}
impl PrintedLine {
    fn new(row: usize, length: usize, next_line_char: char) -> PrintedLine {
        PrintedLine {
            length,
            row,
            next_line_char,
        }
    }
}

struct PrintInfo {
    lines: Vec<PrintedLine>,
    next_ch_i: Option<usize>
}
impl PrintInfo {
    fn new(lines: Vec<PrintedLine>, next_ch_i: Option<usize>) -> Self {
        PrintInfo{lines, next_ch_i}
    }
}

fn main() {
    let screen_info = ScreenInfo {
        width: 130,
        height: 36,
    };
    let chars = read_file("test/test.txt").unwrap();
    let info = print(&chars, &screen_info);
    println!();
    //for l in lines.iter() {
    //    println!("{:?}", l);
    //}
}

fn read_file(path: &str) -> Result<Vec<char>, String> {
    let file_str = std::fs::read_to_string(path).map_err(|e| format!("read file error {}", e))?;
    Ok(file_str.chars().collect())
}

fn print(print_buf: &Vec<char>, screen_info: &ScreenInfo) -> PrintInfo {
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
            let line_with_w = line_len + w_len + del_len;
            let on_new_line = if del == '\n' {
                (true)
            } else if line_with_w > screen_info.width {
                (true)
            } else if line_with_w <= screen_info.width {
                (false)
            } else {
                unreachable!();
            };
            if on_new_line {
                lines.push(PrintedLine::new(row, line_len, del));
                if row == screen_info.height {
                    return PrintInfo::new(lines, Some(i));
                }
                row += 1;
                line_len = w_len;
                print!("\n");
            } else {
                line_len += w_len + del_len;
                if del != '\0' {
                    print!("{}", del);
                }
            }
            if w_len != 0 {
                for c in print_buf[w_i.unwrap()..w_i.unwrap() + w_len].iter() {
                    print!("{}", c);
                }
            }
            if i == print_buf.len() - 1 {
                lines.push(PrintedLine::new(row, line_len, '\0'));
            }
            //print!("{},{}", w_len, line_len);
            w_i = None;
            w_len = 0;
            del = c;
            del_len = if c == '\n' { 0 } else { 1 };
        }
    }

    PrintInfo::new(lines, None)
}
