use ::serde_derive::Deserialize;
use ::std::collections::HashMap;
use ::std::fs;
use ::std::io;

fn words_count() -> Result<(), io::Error> {
    let file_string = fs::read_to_string("p1.txt")?;
    let lowered_string = file_string;

    let mut map = HashMap::new();
    let mut max_len = 0;

    for word in lowered_string.split_whitespace() {
        let mut mod_word = word.replace(".", "");
        mod_word = mod_word.to_lowercase();
        max_len = max_len.max(mod_word.len());
        *map.entry(mod_word).or_insert(0) += 1;
    }

    let mut vec: Vec<(&String, &i32)> = Vec::new();

    for key_val in &map {
        vec.push(key_val);
    }
    vec.sort_by_key(|i| i.1);
    vec.reverse();
    for touple in &vec {
        if touple.0.len() != max_len {
            println!("{:8} => {}", touple.0, touple.1); //pot in loc de 8(care e hardcodat sa pun o variabila?)
        } else {
            println!("{} => {}", touple.0, touple.1);
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Main {
    //count : u32,
    results: Vec<Spell>,
}
#[derive(Debug, Deserialize)]
struct Spell {
    //index : String,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct SpellData {
    name: String,
    level: u32,
    desc: Vec<String>,
}
fn spells() -> Result<(), ureq::Error> {
    let body: String = ureq::get("https://www.dnd5eapi.co/api/spells")
        .call()?
        .into_string()?;

    let first: Main = serde_json::from_str::<Main>(&body).unwrap();

    println!("Introduceti un spell de la tastatura:");
    let mut buffer = String::new(); //from("Wall");
    io::stdin().read_line(&mut buffer)?;
    //buffer.remove(buffer.len() - 1);
    //buffer.shrink_to_fit();

    let buffer = buffer.trim();
    let buffer = buffer.to_lowercase();

    println!("Ati introdus de la tastatura: {}\n", buffer);

    let cpy: &str = &buffer;
    let mut spell_counter = 0;
    for spell_type in &first.results {
        //println!("{:?}", spell_type);
        let spell_name_upper = spell_type.name.to_lowercase();
        if spell_name_upper.contains(cpy) == true {
            spell_counter += 1;
            let url: &str = spell_type.url.as_str();
            let mut spell_api = String::from("https://www.dnd5eapi.co");
            spell_api.push_str(url);

            let body2: String = ureq::get(&spell_api).call()?.into_string()?;

            let main2: SpellData = serde_json::from_str::<SpellData>(&body2).unwrap();

            print!(
                "{}. Name: {}\nLevel: {}\nDescription: ",
                spell_counter, main2.name, main2.level
            ); //, main2.desc);

            for desc in &main2.desc {
                println!("{}", desc);
            }
            println!();

            // let spell_data : Main = serde_json::from_str::<_>(&body2).unwrap();
            //println!("{:?}", spell_type);
        }
    }
    Ok(())
}

fn main() {
    //P1
    words_count().unwrap();

    //BONUS
    spells().unwrap();
}
