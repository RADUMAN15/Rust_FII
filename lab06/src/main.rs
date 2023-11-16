use rusqlite::{Connection, Result};
use std::fs;
use std::io;
use std::vec;

trait Commandtrait {
    fn get_name(&self) -> String;
    fn exec(&mut self, line_args: &Vec<&str>) -> Result<()>;
}

struct PingCommand {}
impl Commandtrait for PingCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("ping");
        x
    }
    fn exec(&mut self, line_args: &Vec<&str>) -> Result<()> {
        if line_args.len() == 0 {
            println!("pong!");
        }
        Ok(())
    }
}

struct CountCommand {}
impl Commandtrait for CountCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("count");
        x
    }
    fn exec(&mut self, line_args: &Vec<&str>) -> Result<()> {
        //println!("DEBUG : {:?}", line_args);
        println!("counted {} elements", line_args.len());
        Ok(())
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

    fn exec(&mut self, line_args: &Vec<&str>) -> Result<()> {
        if line_args.len() == 0 {
            println!("function was called for {} times", self.counter);
            self.counter = self.counter + 1;
        }
        Ok(())
    }
}
#[derive(Debug)]
struct BmData {
    name: String,
    url: String,
}

struct BmCommand {}
impl Commandtrait for BmCommand {
    fn get_name(&self) -> String {
        let x: String = String::from("bm");
        x
    }
    fn exec(&mut self, line_args: &Vec<&str>) -> Result<()> {
        if line_args.len() > 0 {
            if line_args[0] == "add" {
                let conn = Connection::open("bookmarks.db")?;

                let create = r"
                CREATE TABLE IF NOT EXISTS bookmarks (
                    name text not null,
                    url  text not null
                );
                ";
                conn.execute(create, ())?;

                let to_insert = BmData {
                    name: line_args[1].to_string(),
                    url: line_args[2].to_string(),
                };

                conn.execute(
                    "INSERT OR REPLACE INTO bookmarks (name, url) VALUES (?1, ?2)",
                    (&to_insert.name, &to_insert.url),
                )?;
            } else if line_args[0] == "search" {
                let conn = Connection::open("bookmarks.db")?;

                let mut stmt = conn.prepare("SELECT name, url FROM bookmarks")?;
                let name_url_iterator = stmt.query_map([], |row| {
                    Ok(BmData {
                        name: row.get(0)?,
                        url: row.get(1)?,
                    })
                })?;

                println!("Search output for key:{}", line_args[1]);
                for key_val in name_url_iterator {
                    let touple = key_val.unwrap();
                    if touple.name.contains(line_args[1]) || touple.url.contains(line_args[1]) {
                        println!("NAME : URL -> {} {}", touple.name, touple.url);
                    }
                }
            }
        }
        Ok(())
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
                        drop_database_table().unwrap(); //sterg tabelul deoarece daca tot rulez imi adauga inregistrari noi : )
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
                        commands.exec(&arguments).unwrap();
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

fn drop_database_table() -> Result<()> {
    let conn = Connection::open("bookmarks.db")?;

    let delete = r"
                DROP TABLE bookmarks;
                ";
    conn.execute(delete, ())?;
    Ok(())
}
fn main() {
    let mut terminal = Terminal::new();

    // Register commands
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { counter: 0 }));
    terminal.register(Box::new(BmCommand {}));

    // Run the terminal
    if let Err(err) = terminal.run() {
        eprintln!("Error: {}", err);
    }
}
