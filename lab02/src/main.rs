fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i: i32 = n;
    while i > 0 {
        s.push(c);
        i = i - 1;
    }
    return s;
}
fn add_chars_n_ref(s: &mut String, c: char, n: i32) {
    let mut i: i32 = n;
    while i > 0 {
        s.push(c);
        i = i - 1;
    }
}
fn add_space(s: &mut String, n: i32) {
    let mut i: i32 = n;
    while i > 0 {
        s.push(' ');
        i = i - 1;
    }
}
//add_str: concatenates a str to the string
fn add_str(s: &mut String, add: &str) {
    s.push_str(add);
}

//add_integer: concatenates an integer to the string. Add separators at every 3 digits.
fn add_integer(s: &mut String, mut n: u32) {
    let mut point: i32 = 0;
    let len = s.len();

    if n == 0 {
        s.insert(len, '0');
    }
    while n > 0 {
        if (point) % 3 == 0 && point != 0 {
            s.insert(len, '_');
        }
        point = point + 1;
        let x: u8 = (n % 10) as u8;
        n = n / 10;

        s.insert(len, (x + 48) as char);
    }
}
//add_float: concatenates a float to the string
fn add_float(s: &mut String, n: f32) {
    /*let mut n_floored: i32 = n.floor() as i32;  //pt a lua partea intreaga
    let mut remain : f32 = n - n_floored as f32; //pt a lua partea fractionara
    let len = s.len();
    while n_floored > 0{
        let x: u8 = (n % 10.0) as u8;
        n_floored = n_floored / 10;
        s.insert(len, (x + 48) as char);
    }
    s.insert(len + 1, '.');

    let mut i: i32 = 5; //aleg primele 5 zecimale
    while i > 0{

        remain = remain * 10.0;
        i  = i - 1;
    }
    //scot zecimalele de la final de care nu am nevoie

    let mut remain_floor: i32 = remain.floor() as i32;
    while remain_floor % 10 == 0{
        remain_floor = remain_floor / 10;
    }
    //partea zecimala -> tot crapa daca am un 0 in fata
    while remain_floor > 0 {

        let x: u8 = (remain_floor % 10) as u8;
        remain_floor = remain_floor / 10;
        s.insert(len + 2, (x + 48) as char);
    }
    */
    s.push_str(&n.to_string()); //-> pt a obtine ce trebuie
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    print!("{}", s);

    println!("\n\n");

    //P2
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_ref(&mut s, c, 26 - i);

        i += 1;
    }
    print!("{}", s);

    println!("\n\n");
    //P3
    let mut s: String = String::new();
    add_space(&mut s, 40);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "latest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 8);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");
    print!("{}", s);
}
