use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct ProcessData {
    pid : u32,
    ppid : u32,
    name: String,
    cpu_used: u32,
    memory_used: u32,
    path: String,
    author: String,
}
trait Methods {
    fn new(
        pid : u32,
        ppid : u32,
        name: String,
        cpu_used: u32,
        memory_used: u32,
        path: String,
        author: String,
    ) -> Self;
}
impl Methods for ProcessData {
    fn new(
        pidi : u32,
        ppidi : u32,
        namei: String,
        cpu_usedi: u32,
        memory_usedi: u32,
        pathi: String,
        authori: String,
    ) -> Self {
        ProcessData {

            pid : pidi,
            ppid : ppidi,
            name: namei,
            cpu_used: cpu_usedi,
            memory_used: memory_usedi,
            path: pathi,
            author: authori,
        }
    }
}
fn main() {

    //K = PID, V = LIST OF SUBPROCESSES
    //let mut _process_list : HashMap<u32, Vec<ProcessData>> = HashMap::new();

    printproc().unwrap();

    printglobal().unwrap();
}
fn get_username(uid: u32) -> Result<String, io::Error> {
    let passwd_file_path: &str = "/etc/passwd";
    let passwd_string: String = fs::read_to_string(passwd_file_path)?;

    for line in passwd_string.lines() {
        let args: Vec<&str> = line.split(':').collect();

        let user_name = args[0];
        if let Ok(line_uid) = args[2].parse::<u32>() {
            if line_uid == uid {
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

fn printproc() -> Result<HashMap<u32, Vec<u32>>, io::Error> {

    let mut processes: Vec<ProcessData> = Vec::new();

    let mut _process_list : HashMap<u32, Vec<u32>> = HashMap::new();

    let paths: fs::ReadDir = fs::read_dir("/proc").unwrap();
    for path in paths {
        let file_name: std::ffi::OsString = path.unwrap().file_name();
        if let Some(name) = file_name.to_str() {
            if name.parse::<f64>().is_ok() {
                let mut path_status_file: String = String::from("/proc/");
                path_status_file.push_str(name);
                path_status_file.push_str("/status");
                let proc_status: String = fs::read_to_string(path_status_file)?;

                let mut uid: u32 = 0;

                let mut add: ProcessData = ProcessData::new(
                    0,
                    0,
                    "".to_string(),
                    0,
                    0,
                    "".to_string(),
                    "".to_string(),
                );
                for line in proc_status.lines() {
                    if line.contains("Name") {
                        //println!("{line}");
                        add.name = line[6..].to_string();
                    }
                    if line.starts_with("Pid") {
                        //println!("{line}");
                        let str = line[5..].to_string().replace(' ', "");
                        add.pid = str.parse::<u32>().unwrap();
                    }
                    if line.contains("PPid") {
                        //println!("{line}");
                        let str = line[6..].to_string().replace(' ', "");
                        add.ppid = str.parse::<u32>().unwrap();
                    }
                    //if line.starts_with("Pid"){println!("{line}");}
                    if line.contains("VmSize") {
                        //println!("{line}");
                        let mut str = line[8..].to_string().replace(' ', "");
                        str = str.replace("kB", "");
                        add.memory_used = str.parse::<u32>().unwrap();
                    }
                    if line.contains("Uid") {
                        //Uid =
                        let args: Vec<&str> = line.split_whitespace().collect();
                        //println!("{:?}", args);
                        if let Ok(uid_converted) = args.get(1).unwrap().parse::<u32>() {
                            uid = uid_converted;
                        }
                        //println!("{line}");
                        //println!("Uid : {uid}");
                        let process_author_username = get_username(uid).unwrap();
                        //println!("Username : {process_author_username}");

                        add.author = process_author_username;
                    }
                }
                //Uid
                let mut path_stat_file: String = String::from("/proc/");
                path_stat_file.push_str(name);
                path_stat_file.push_str("/stat");

                let proc_status: String = fs::read_to_string(path_stat_file)?;

                let status_args: Vec<&str> = proc_status.split_whitespace().collect();

                if let Some(utime) = status_args.get(13) {
                    if let Some(stime) = status_args.get(14) {
                        if let Some(starttime) = status_args.get(21) {
                            let path_uptime_file: String = String::from("/proc/uptime");
                            let proc_uptime: String = fs::read_to_string(path_uptime_file)?;

                            let status_args: Vec<&str> = proc_uptime.split_whitespace().collect();
                            let system_uptime = status_args.first().unwrap();

                            if let Ok(system_uptime_f64) = system_uptime.parse::<f64>() {
                                if let Ok(mut utime_sec) = utime.parse::<f64>() {
                                    utime_sec /= 100.0;
                                    //println!("utime_sec: {}", utime_sec);

                                    if let Ok(mut stime_sec) = stime.parse::<f64>() {
                                        stime_sec /= 100.0;
                                        //println!("utime_sec: {}", stime_sec);

                                        if let Ok(mut starttime_sec) = starttime.parse::<f64>() {
                                            starttime_sec /= 100.0;
                                            //println!("utime_sec: {}", starttime_sec);

                                            let elapsed_time_sec: f64 =
                                                system_uptime_f64 - starttime_sec;
                                            let proc_usage_sec: f64 = utime_sec + stime_sec;
                                            let proc_usage_procents: f64 =
                                                proc_usage_sec * 100.0 / elapsed_time_sec;
                                            let proc_usage_procents = proc_usage_procents as u32;

                                            //println!("CPU usage: {}%", proc_usage_procents);
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
                if let Ok(path_str) = path_file {
                    if let Some(path_ok) = path_str.to_str() {
                        //println!("Path : {}", path_ok);

                        add.path = path_ok.to_string();
                    }
                }
                //println!("\n{:?}\n",add);
                processes.push(add);
                //println!();
            }
        }
    }
    //println!("\n{:?}\n",processes);

    for process in &processes{

        let mut children : Vec<u32> = Vec :: new();
        for ischild in &processes{
            if ischild.ppid == process.pid {
                children.push(ischild.pid);
            }
        }
        _process_list.insert(process.pid, children);
    }

    let mut sorted_vec: Vec<_> = _process_list.into_iter().collect();

    // Sort the vector by keys
    sorted_vec.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, values) in sorted_vec.iter_mut() {
        values.sort();
    }

    for (key, value) in sorted_vec {
        println!("PID: {}, SUB-PIDS: {:?}", key, value);
    }

    //println!("{:?}", _process_list);
    let mut _process_list : HashMap<u32, Vec<u32>> = HashMap::new();
    Ok(_process_list)
}

fn printglobal() -> Result<(), io::Error> {

    let global_proc_stat_path: String = String::from("/proc/stat");
    let processor_stat: String = fs::read_to_string(global_proc_stat_path)?;

    let line = processor_stat.lines().next().unwrap();

    let args : Vec<&str> = line.split_whitespace().collect();
    let mut sum : u32 = 0;
    for arg in &args{

        if arg.parse::<u32>().is_ok(){
            let value = arg.parse::<u32>().unwrap();
            sum += value;
        }
    }
    let idle_str = args.get(4).unwrap();
    let idle_u32 = idle_str.parse::<u32>().unwrap();

    let global_usage = (100 * idle_u32) / sum;
    println!("CPU USAGE: {:?}%",global_usage);

    Ok(())
}