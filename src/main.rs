use std::fs::File;
use std::io::BufRead;

struct ScreenInfo {
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Line<'a> {
    slice: &'a[char],
    ending: char
}
impl <'a> Line<'a> {
    fn new(slice: &'a[char], ending: char) -> Self {
        Line { slice, ending }
    }
}

fn main() {
    let screen_info = ScreenInfo {
        width: 130,
        height: 36,
    };
    let chars = read_file("test/test.txt").unwrap();
    let lines = read_lines(&chars, 0, &screen_info);
    let mut it = 0;
    for l in lines.iter() {
       // print!("$");
        print!("{}:", it);
        it += 1;
        for c in l.slice.iter() {
           // print!("{}", c);
        }
        //println!();
    }

}

fn read_file(path: &str) -> Result<Vec<char>, String> {
    let file_str = std::fs::read_to_string(path).map_err(|e| format!("read file error {}", e))?;
    Ok(file_str.chars().collect())
}

fn read_lines<'a>(print_buf: &'a Vec<char>, offset_ch_i: usize, screen_info: &ScreenInfo) -> Vec<Line<'a>> {
    //word
    let mut w_i = None;
    let mut w_len = 0;
    //word delimeter
    let mut del = '\0';
    let mut del_len = 0;
    //line
    let mut line_len = 0;
    let mut lines = Vec::new();

    let mut row = 0;

    for (i, &c) in print_buf[offset_ch_i..].iter().enumerate() {
        let i = i + offset_ch_i;
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
                
               // println!(" {} {} ", row, w_len);
                lines.push(Line::new(&print_buf[i - 1 - line_len..i], del));
                print!("len: {}:", line_len);
                for c in print_buf[i - 1 - line_len..i].iter() {
                    print!("{}", c);
                }
                println!();
                
                if row == screen_info.height {
                    break;
                }
                row += 1;
                line_len = w_len;
                //print!("\n");
            } else {
                line_len += w_len + del_len;
                //if del != '\0' {
                //    print!("{}", del);
                //}
            }
            //if w_len != 0 {
            //    for c in print_buf[w_i.unwrap()..w_i.unwrap() + w_len].iter() {
            //        print!("{}", c);
            //    }
            //}
            if i == print_buf.len() - 1 {
                lines.push(Line::new(&print_buf[i - line_len -1..i], '\0'));
                for c in print_buf[i - 1 - line_len..i].iter() {
                    print!("{}", c);
                }
            }
            
            //print!("{},{}", w_len, line_len);
            w_i = None;
            w_len = 0;
            del = c;
            del_len = if c == '\n' { 0 } else { 1 };
        }
    }

    lines
}
