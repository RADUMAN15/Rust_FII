use std::fs;
use std::io;

#[derive(Debug,Clone)]
struct ProcessData{

    name : String,
    cpu_used : u32,
    memory_used : String,
    path : String,
    author : String,
}
trait Methods{
    fn new(name : String,
        cpu_used : u32,
        memory_used : String,
        path : String,
        author : String,) -> ProcessData;
}
impl Methods for ProcessData{

    fn new(namei : String,
        cpu_usedi : u32,
        memory_usedi : String,
        pathi : String,
        authori : String,) -> ProcessData{

                ProcessData{name : namei,
                        cpu_used : cpu_usedi,
                        memory_used : memory_usedi,
                        path : pathi,
                        author : authori,}
        }
}
fn main() {

    printproc().unwrap();
}
fn get_username(uid : u32) -> Result< String, io::Error>{

    let passwd_file_path: &str = "/etc/passwd";
    let passwd_string: String = fs::read_to_string(passwd_file_path)?;

    for line in passwd_string.lines(){

        let args : Vec<&str>= line.split(':').collect();

        let user_name = args[0];
        if let Ok(line_uid) = args[2].parse::<u32>(){

            if line_uid == uid{
               // user_name_string.push_str(user_name);
                return Ok(user_name.to_string());
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "UID not found in /etc/passwd",
    ))

}

fn printproc() -> Result<(), io::Error> {

    //let mut processes : Vec<ProcessData> = Vec::new();

    let paths: fs::ReadDir = fs::read_dir("/proc").unwrap();
    for path in paths {

        let file_name: std::ffi::OsString = path.unwrap().file_name();
        if let Some(name) = file_name.to_str() {
            if name.parse::<f64>().is_ok()
            {
                let mut path_status_file: String = String::from("/proc/");
                path_status_file.push_str(name);
                path_status_file.push_str("/status");
                let proc_status: String = fs::read_to_string(path_status_file)?;

                let mut uid : u32 = 0;

                let mut add : ProcessData =ProcessData::new("".to_string(), 0, "0K".to_string(), "".to_string(), "".to_string());
                for line in proc_status.lines(){

                    if line.contains("Name"){

                        println!("{line}");
                        add.name = line[6..].to_string();
                    }
                    //if line.starts_with("Pid"){println!("{line}");}
                    if line.contains("VmSize"){
                        println!("{line}");
                        add.memory_used = line[9..].to_string().replace(" ", "");

                    }
                    if line.contains("Uid"){

                        //Uid = 
                        let args : Vec<&str> = line.split_whitespace().collect();
                        //println!("{:?}", args);
                        if let Ok(uid_converted) = args.get(1).unwrap().parse::<u32>(){
                            uid = uid_converted;
                        }
                        //println!("{line}");
                        //println!("Uid : {uid}");
                        let process_author_username = get_username(uid).unwrap();
                        println!("Username : {process_author_username}");

                        add.author = process_author_username;
                    }

                }     
                //Uid            
                let mut path_stat_file: String = String::from("/proc/");
                path_stat_file.push_str(name);
                path_stat_file.push_str("/stat");

                let proc_status: String = fs::read_to_string(path_stat_file)?;
                
                let status_args : Vec<&str> = proc_status.split_whitespace().collect();
                
                if let Some(utime) = status_args.get(13){

                    if let Some(stime) = status_args.get(14){

                        if let Some(starttime) = status_args.get(21){

                            let path_uptime_file: String = String::from("/proc/uptime");
                            let proc_uptime: String = fs::read_to_string(path_uptime_file)?;
                            
                            let status_args : Vec<&str>= proc_uptime.split_whitespace().collect();
                            let system_uptime = status_args.first().unwrap();

                            if let Ok(system_uptime_f64) = system_uptime.parse::<f64>(){
                                if let Ok(mut utime_sec) = utime.parse::<f64>(){
                                    utime_sec /= 100.0;
                                    //println!("utime_sec: {}", utime_sec);
    
                                    if let Ok(mut stime_sec) = stime.parse::<f64>(){
                                        stime_sec /= 100.0;
                                        //println!("utime_sec: {}", stime_sec);
    
                                        if let Ok(mut starttime_sec) = starttime.parse::<f64>(){
                                            starttime_sec /= 100.0;
                                            //println!("utime_sec: {}", starttime_sec);
    
                                            let elapsed_time_sec: f64 = system_uptime_f64 - starttime_sec;
                                            let proc_usage_sec: f64 = utime_sec + stime_sec;
                                            let proc_usage_procents: f64 = proc_usage_sec * 100.0 / elapsed_time_sec;
                                            let proc_usage_procents = proc_usage_procents as u32;

                                            println!("CPU usage: {}%", proc_usage_procents);
                                            add.cpu_used = proc_usage_procents;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                //PathFile
                let mut path_proc_file: String = String::from("/proc/");
                path_proc_file.push_str(name);
                path_proc_file.push_str("/exe");
                let path_file = fs::read_link(path_proc_file);
                if let Ok(path_str) = path_file{
                    if let Some(path_ok) = path_str.to_str(){
                        println!("Path : {}", path_ok);

                        add.path = path_ok.to_string();
                    }
                }
                println!("\n{:?}\n",add);

                println!();



            }   
        }        
    }
    Ok(())
}