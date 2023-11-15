use std::fs;
use std::io;
use std::vec;
trait Commandtrait {
    fn get_name(&self) -> String;
    fn exec(&mut self, line_args: &Vec<&str>);
}

struct PingCommand {}
impl Commandtrait for PingCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("ping");
        x
    }
    fn exec(&mut self, line_args: &Vec<&str>) {
        if line_args.len() == 0 {
            println!("pong!");
        }
    }
}

struct CountCommand {}
impl Commandtrait for CountCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("count");
        x
    }
    fn exec(&mut self, line_args: &Vec<&str>) {
        //println!("DEBUG : {:?}", line_args);
        println!("counted {} elements", line_args.len());
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

    fn exec(&mut self, line_args: &Vec<&str>) {
        if line_args.len() == 0 {
            println!("function was called for {} times", self.counter);
            self.counter = self.counter + 1;
        }
    }
}
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

    fn run(&mut self) -> Result<(), io::Error> {
        let file_commands = fs::read_to_string("command_file.txt")?;

        'main_loop: for command_line in file_commands.lines() {
            if command_line != "\n" && !command_line.starts_with(" ") {
                //iau argumentele din comanda si le pun in vectorul v
                let command_args = command_line;
                let mut v: Vec<&str> = Vec::new();
                for arg in command_args.split_whitespace() {
                    v.push(arg);
                }
                //verific daca comanda exista in collection
                //println!("DEBUG: {:?}", v);
                let mut command_exists: bool = false;
                for commands in &mut self.collection {
                    let collection_commands = commands.get_name();

                    if v.len() > 0 && v[0] == collection_commands {
                        command_exists = true;
                    } else if v.len() > 0 && v[0] == "stop" {
                        break 'main_loop;
                    }

                    if command_exists {
                        //command exists in recollection

                        let mut arguments: Vec<&str> = Vec::new();
                        let mut it = 0;

                        for args in &v {
                            if it != 0 && !args.contains(" ") {
                                arguments.push(args);
                            }
                            it = it + 1;
                        }

                        let commands: &mut dyn Commandtrait = &mut **commands;
                        commands.exec(&arguments);
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
fn main() {
    let mut terminal = Terminal::new();

    // Register commands
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { counter: 0 }));
    // Run the terminal
    if let Err(err) = terminal.run() {
        eprintln!("Error: {}", err);
    }
}
