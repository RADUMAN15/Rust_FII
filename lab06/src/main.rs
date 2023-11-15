use std::fs;
use std::io;
use std::vec;

use alloc::collections;

trait Commandtrait {
    fn get_name(&self) -> String;
    fn exec(&mut self, line_args: Vec<&str>);
}

struct PingCommand {}
impl Commandtrait for PingCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("ping");
        x
    }
    fn exec(&mut self, _line_args: Vec<&str>) {
        print!("pong!");
    }
}

struct CountCommand {}
impl Commandtrait for CountCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("count");
        x
    }
    fn exec(&mut self, line_args: Vec<&str>) {
        let mut v: Vec<&str> = Vec::new();
        for arg in line_args.split(' ') {
            if arg.cmp(" ").is_ne() {
                //nu iau si spatiile
                v.push(arg);
            }
        }

        if v[0].cmp("count").is_ne() {
            //nimic
        } else {
            //am comanda count
            print!("counted {} args", v.len() - 2); //bug daca am mai multe spatii intre cuvinte (fix?)
        }
    }
}

struct TimesCommand {
    counter: u32,
}
impl Commandtrait for TimesCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("times");
        x
    }

    fn exec(&mut self, line_args:Vec<&str> ) {
        if line_args.cmp("times").is_eq() {
            self.counter += 1;
            print!("count called for {} times", self.counter);
        }
    }
}

/*struct StopCommand{

}
*/

struct Terminal {
    collection: Vec<Box<dyn Commandtrait>>,
}

impl Terminal {

    fn new() -> Terminal {
        Terminal { collection: vec![] } //e ok ?
    }

    fn register(&mut self, new_command: Box<dyn Commandtrait>) {
        self.collection.push(new_command);
    }

    fn run(&self) -> Result<(), io::Error> {

        let file_commands = fs::read_to_string("command_file.txt")?;
        
        for command_line in file_commands.lines() {

            //parsez comanda
            let command_args  = command_line;
            let mut v: Vec<&str> = Vec::new();
            for arg in command_args.split_whitespace(){
                v.push(arg);
            }   
            let mut command_exists : bool= false;
            for commands in self.collection{

                let collection_commands = commands.get_name();
                if v[0].cmp(&collection_commands).is_eq(){
                    command_exists = true;
                }
                if command_exists{ //command exists in recollection

                    let mut args_ok : bool = true;
                    let mut arguments: Vec<&str> = Vec::new();
                    for args in &v{
                        if args.contains(" "){ //arg nu sunt valide
                            args_ok = false;
                        }
                    }
                    if args_ok{
                        let mut it = 0;
                        for args in &v{
                            if it != 0{
                                arguments.push(args);
                            }
                            it = it + 1;
                    }
                    
                    commands.exec(arguments);
                }

            } 
        }

        Ok(())
    }
}
fn main() {
    
    
}
