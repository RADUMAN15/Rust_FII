use std::fs;
use std::io;

fn compute_max_line() -> Result<(), io::Error> {
    let mut max_len: (i32, i32) = (0, 0);
    let mut line_max_ch: String = String::new();
    let mut line_max_bt: String = String::new();
    let mut line_ch_len: i32;
    let mut line_bt_len: i32;

    let s = fs::read_to_string("text.txt")?;
    for line in s.lines() {
        line_ch_len = line.chars().count() as i32;
        line_bt_len = line.len() as i32;

        if max_len.0 < line_bt_len {
            max_len.0 = line_bt_len;
            line_max_bt = line.to_string();
        }
        if max_len.1 < line_ch_len {
            max_len.1 = line_ch_len;
            line_max_ch = line.to_string();
        }
    }
    println!(
        "Max line size in bytes: {} with size:{}",
        line_max_bt, max_len.0
    );
    println!(
        "Max line size in chars: {} with size:{}",
        line_max_ch, max_len.1
    );
    return Ok(());
}

fn rot13(s: &String) -> String {
    let mut result: String = String::new();

    for c in s.chars() {
        let mut ch: u8 = c as u8;
        if !ch.is_ascii() {
            panic!("Encountered char: {} is not ASCII", ch);
        } else {
            if ch >= 'a' as u8 && ch <= 'm' as u8 {
                ch = (ch as u32 + 13) as u8;
                result.push(ch as char);
            } else if ch >= 'n' as u8 && ch <= 'z' as u8 {
                ch = ('a' as u32 + (ch as u32 + 12) % ('z' as u32)) as u8;
                result.push(ch as char);
            } else if ch >= 'A' as u8 && ch <= 'M' as u8 {
                ch = (ch as u32 + 13) as u8;
                result.push(ch as char);
            } else {
                ch = ('A' as u32 + (ch as u32 + 12) % ('Z' as u32)) as u8;
                result.push(ch as char);
            }
        }
    }
    return result;
}

fn no_abrev() -> Result<(), io::Error> {
    let mut ans: String = String::new();
    let abr: (&str, &str, &str, &str, &str, &str, &str, &str) = (
        "pentru", "pt", "pentru", "ptr", "domnul", "dl", "doamna", "dna",
    );
    let s = fs::read_to_string("propozitie.txt")?;
    println!("Initial string is: {}", s);

    for v in s.split(' ') {
        if v.starts_with(abr.1) && v.ends_with(abr.1) {
            ans.push_str(abr.0);
        } else if v.starts_with(abr.3) && v.ends_with(abr.3) {
            ans.push_str(abr.2);
        } else if v.starts_with(abr.5) && v.ends_with(abr.5) {
            ans.push_str(abr.4);
        } else if v.starts_with(abr.7) && v.ends_with(abr.7) {
            ans.push_str(abr.6);
        } else {
            ans.push_str(v);
        }
        ans.push(' ');
    }
    println!("Modified string is: {}", ans);

    Ok(())
}

fn host_print() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:Windows\\System32\\drivers\\etc\\hosts")?;
    let mut good_to_parse: bool = false;
    let mut i: i32;
    println!("Hosts from hosts file:");
    for line in s.lines() {
        if line.contains("# localhost name resolution is handled within DNS itself") {
            good_to_parse = true;
            continue;
        }
        if good_to_parse {
            i = 0;
            for columns in line.split(' ') {
                if !columns.contains(' ') && columns.len() > 3 {
                    if i < 1 {
                        print!("{} => ", &columns[2..]);
                        i = i + 1;
                    } else {
                        println!("{columns}");
                    }
                }
            }
        }
    }
    Ok(())
}
fn main() {
    //P1
    compute_max_line().unwrap();

    //P2
    let mut s = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    println!("ROT13(DECODE):{s}");
    s = rot13(&s);
    println!("ROT13(ENCODE):{s}");
    s = rot13(&s);
    println!("ROT13(DECODE):{s}");
    //panic
    //s = String::from("abc日本人LL");
    //println!("{}", rot13(&s));

    //P3
    no_abrev().unwrap();

    //P4
    host_print().unwrap();
}
