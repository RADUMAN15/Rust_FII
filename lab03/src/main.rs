fn next_prime(x: u16) -> Option<u16> {
    let mut copy_x: u16 = x + 1;

    while copy_x < u16::MAX {
        let mut ok = true;
        //check if is prime
        if copy_x < 2 {
            //continue;
        }
        let mut d: u32 = 2;
        while d * d <= copy_x as u32 {
            //if d was u16 -> it produced overflow on u16::MAX^2 when it was compared with copy_x (u16)
            if copy_x as u32 % d == 0 {
                ok = false;
                break;
            }
            d = d + 1;
        }

        if ok == true {
            return Some(copy_x);
        }
        copy_x = copy_x + 1; //search for next prime
    }

    return None;
}

fn add_u32(a: u32, b: u32) {
    let sum: u64 = a as u64 + b as u64;
    if sum <= u32::MAX as u64 {
        println!("The sum of {} and {} is: {}", a, b, a + b);
    } else {
        panic!("Expected a + b to be smaller than u32");
    }
}
fn mul_u32(a: u32, b: u32) {
    let mul: u64 = a as u64 * b as u64;
    if mul <= u32::MAX as u64 {
        println!("The product of {} and {} is: {}", a, b, a * b);
    } else {
        panic!("Expected a * b to be smaller than u32");
    }
}
fn add_u32_result(a: u32, b: u32) -> Result<u32, &'static str> {
    let sum: u64 = a as u64 + b as u64;
    if sum <= u32::MAX as u64 {
        return Ok(a + b);
    } else {
        return Err("The sum overflows");
    }
}
fn mul_u32_result(a: u32, b: u32) -> Result<u32, &'static str> {
    let mul: u64 = a as u64 * b as u64;
    if mul <= u32::MAX as u64 {
        return Ok(a * b);
    } else {
        return Err("The product overflows");
    }
}

fn muladdmul(a: u32, b: u32, c: u32, d: u32) -> Result<u32, &'static str> {
    let prduct1 = mul_u32_result(a, b)?;
    let prduct2 = mul_u32_result(c, d)?;
    let sum_of_prod = add_u32_result(prduct1, prduct2)?;
    Ok(sum_of_prod)
}

#[derive(Debug)]
enum CharError {
    NotAscii,
    NotDigit,
    NotB16Digit,
    NotLetter,
    NotPrintable,
}
fn to_uppercase(c: char) -> Result<char, CharError> {
    if c.is_ascii() && c.is_alphabetic() {
        return Ok(c.to_ascii_uppercase());
    } else {
        return Err(CharError::NotAscii);
    }
}
fn to_lowercase(c: char) -> Result<char, CharError> {
    if c.is_ascii() && c.is_alphabetic() {
        return Ok(c.to_ascii_lowercase());
    } else {
        return Err(CharError::NotLetter);
    }
}
fn print_char(c: char) -> Result<char, CharError> {
    if c.is_ascii() && c.is_ascii_graphic() {
        return Ok(c);
    } else {
        return Err(CharError::NotPrintable);
    }
}
fn char_to_number(c: char) -> Result<u32, CharError> {
    if c >= '0' as char && c <= '9' as char {
        return Ok(c.to_digit(10).unwrap());
    } else {
        return Err(CharError::NotDigit);
    }
}
fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if c >= '0' as char && c <= '9' as char
        || c >= 'a' as char && c <= 'f' as char
        || c >= 'A' as char && c <= 'F' as char
    {
        return Ok(c.to_digit(16).unwrap());
    } else {
        return Err(CharError::NotB16Digit);
    }
}
fn print_error(e: CharError) {
    match e {
        CharError::NotAscii => print!("char is not ASCII"),
        CharError::NotB16Digit => print!("char is not a base16 digit"),
        CharError::NotDigit => print!("char is not a digit"),
        CharError::NotLetter => print!("char is not a letter"),
        CharError::NotPrintable => print!("char is not printable"),
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ErrorRaduBonus {
    // for P3
    #[error("sum of u32 and u32 overflows u32")]
    SumOverflow,
    #[error("mul of u32 and u32 overflows u32")]
    MulOverflow,
    #[error("{0} already overflows u32")] //nu prea are sens deoarece functia mea accepta u32
    VariableOverflow(u64),
}
fn sumerr(x: u32, y: u32) -> Result<u32, ErrorRaduBonus> {
    let sum: u64 = x as u64 + y as u64;
    if sum <= u32::MAX as u64 {
        Ok(sum as u32)
    } else {
        Err(ErrorRaduBonus::SumOverflow)
    }
}
fn mulerr(x: u32, y: u32) -> Result<u32, ErrorRaduBonus> {
    let mul: u64 = x as u64 * y as u64;
    if mul <= u32::MAX as u64 {
        Ok(mul as u32)
    } else {
        Err(ErrorRaduBonus::MulOverflow)
    }
}

fn main() {
    //BONUS PART <->
    match sumerr(4_294_967_294, 1) {
        Ok(x) => println!("4_294_967_294 + 1 = {}", x),
        Err(e) => println!("err: {}", e),
    }
    match sumerr(4_294_967_294, 2) {
        Ok(x) => println!("4_294_967_294 + 2 = {}", x),
        Err(e) => println!("err: {}", e),
    }
    match mulerr(4_294_967_294, 1) {
        Ok(x) => println!("4_294_967_294 * 1 = {}", x),
        Err(e) => println!("err: {}", e),
    }
    match mulerr(4_294_967_294, 2) {
        Ok(x) => println!("4_294_967_294 * 2 = {}", x),
        Err(e) => println!("err: {}", e),
    }
    //P1

    let value: Option<u16> = next_prime(2); //next is 65537 -> over u16::MAX
                                            //let value: Option<u16> = next_prime(65532);//next is 65537 -> over u16::MAX
    if value.is_some() {
        println!("the next prime number is {}", value.unwrap());
    } else {
        println!("Error -> overflow u16");
    }

    //P2

    add_u32(4_294_967_294, 1); //works sum is u32::MAX
                               //add_u32(4_294_967_295, 1);  //panics
    mul_u32(858993459, 5); // works -> product is u32::MAX
                           //mul_u32(858993459, 6); //panics

    //P3

    let result1: Result<u32, &str> = add_u32_result(4_294_967_294, 1);
    if result1.is_err() {
        println!("Error found: {}", result1.err().unwrap());
    } else {
        println!("Success, result is {}", result1.ok().unwrap());
    }
    let result2: Result<u32, &str> = mul_u32_result(4_294_967_294, 1);
    if result2.is_err() {
        println!("Error found: {}", result2.err().unwrap());
    } else {
        println!("Success, result is {}", result2.ok().unwrap());
    }

    //propagating error in this fn
    //let x: (u32, u32, u32, u32) = (10, 20, 30 , 0xFFFFFFFF); //err
    let x: (u32, u32, u32, u32) = (10, 20, 30, 40);
    match muladdmul(x.0, x.1, x.2, x.3) {
        Ok(res) => println!(
            "The sum of products of ({},{}) and ({},{}) is {}",
            x.0, x.1, x.2, x.3, res
        ),
        Err(e) => println!("err is : {}", e),
    }

    //P4

    let mut c: char = 'a' as char;
    match to_uppercase(c) {
        Ok(res) => println!("Uppercase of {} is {}", c, res),
        Err(error) => print_error(error),
    }

    c = 'A' as char;
    match to_lowercase(c) {
        Ok(res) => println!("Lowercase of {} is {}", c, res),
        Err(error) => print_error(error),
    }

    c = '5' as char;
    match char_to_number(c) {
        Ok(res) => println!("u32 values of '{}' is {}", c, res),
        Err(error) => print_error(error),
    }

    c = 'F' as char;
    match char_to_number_hex(c) {
        Ok(res) => println!("HEX values of '{}' is {}", c, res),
        Err(error) => print_error(error),
    }

    c = '$'; //💚©
    match print_char(c) {
        Ok(res) => println!("Aspect of ASCII char '{}' is {}", c, res),
        Err(error) => print_error(error),
    }

    //P5
    
}
