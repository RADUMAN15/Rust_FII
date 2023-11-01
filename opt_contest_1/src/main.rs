use std::fs;
use std::io;

fn rot13() -> Result<(), io::Error> {
    let s = fs::read_to_string("input.txt")?;
    let mut result: String = String::new();
    let mut ch;
    for c in s.chars() {
        ch = c as u8;
        if !c.is_ascii() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Char encountered is not ASCII",
            ));
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
            } else if ch >= 'N' as u8 && ch <= 'Z' as u8 {
                ch = ('A' as u32 + (ch as u32 + 12) % ('Z' as u32)) as u8;
                result.push(ch as char);
            } else {
                //am orice ascii in loc de litere (nu cast ch la u8 pt a nu pierde biti)
                result.push(c);
            }
        }
    }
    fs::write("output.txt", result)?;
    Ok(())
}
fn main() {
    //Ç███ÆÆ (caracterele sunt ASCII dar vad ca nu mi le vede drept ASCII analizatorul idk dc)

    if let Err(err) = rot13() {
        eprintln!("Error: {:?}", err);
    } else {
        println!("Done!");
    }
}
