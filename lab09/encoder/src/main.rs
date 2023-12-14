use base64::encode;
use std::fs;
use std::io::{self};

fn get_arguments() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect(); 
    if args.len() < 3 {
        println!("Nu am trimis fisiere la cmd\nCitim din linia de comanda:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let encoded = encode(input.trim().as_bytes());
        println!("Encoded: {}", encoded);
    } else if args.len() == 3 {
        println!(
            "intput file : {} output file : {}",
            args.get(1).unwrap(),
            args.get(2).unwrap()
        );

        let input_file: &str = args.get(1).unwrap();
        let output_file: &str = args.get(2).unwrap();

        println!("Read from : {} and encode to: {}", input_file, output_file);

        let content_input: String = fs::read_to_string(input_file)?;
        //let content_output : String = fs::read_to_string(output_file)?;

        let encoded = base64::encode(content_input.as_bytes());

        match fs::write(output_file, encoded) {
            Ok(_) => println!("Data successfully written to {}", output_file),
            Err(err) => eprintln!("Error writing to {}: {}", output_file, err),
        }

        //println!("{encoded}");
    }
    Ok(())
}
fn main() {
    println!("encoder, version {}", env!("CARGO_PKG_VERSION"));
    #[cfg(target_os = "windows")]
    {
        println!("Built for Windows");
    }
    #[cfg(target_os = "linux")]
    {
        println!("Built for Linux");
    }
    #[cfg(target_os = "macos")]
    {
        println!("Built for macOS");
    }
    // let input = b"Hello, World!";
    // let s : String = base64::encode(input);
    // println!("{s}");

    get_arguments().unwrap();
}
