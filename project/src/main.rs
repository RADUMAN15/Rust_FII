use druid::widget::Button;
use druid::widget::Checkbox;
use druid::widget::SizedBox;
use druid::widget::{Align, Flex, Label, Scroll};
use druid::Color;
use druid::UnitPoint;
use druid::WidgetExt;
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
use std::collections::HashMap;
use std::fs;
use std::io;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Task Raduger");
//const HORIZONTAL_WIDGET_SPACING: f64 = 20.0;

#[derive(Debug, Clone)]
struct ProcessData {
    pid: u32,
    ppid: u32,
    name: String,
    cpu_used: f64,
    memory_used: u32,
    path: String,
    author: String,
}
trait Methods {
    fn new(
        pid: u32,
        ppid: u32,
        name: String,
        cpu_used: f64,
        memory_used: u32,
        path: String,
        author: String,
    ) -> Self;
}

impl Methods for ProcessData {
    fn new(
        pidi: u32,
        ppidi: u32,
        namei: String,
        cpu_usedi: f64,
        memory_usedi: u32,
        pathi: String,
        authori: String,
    ) -> Self {
        ProcessData {
            pid: pidi,
            ppid: ppidi,
            name: namei,
            cpu_used: cpu_usedi,
            memory_used: memory_usedi,
            path: pathi,
            author: authori,
        }
    }
}

#[derive(Clone, Data, Lens)]
struct AppState {
    view_name: String,
    view_option: bool, //0 list 1 tree
    view_pname: bool,
    view_cpu: bool,
    view_mem: bool,
    view_usr: bool,
    view_path: bool,
    process_info: String,
    global_cpu: String,
    global_mem: String,
}

fn build_root_widget() -> impl Widget<AppState> {
    let label_view_mode = Label::new(|data: &AppState, _env: &_| {
        // Fix: Use the name field from the AppState.
        format!("ViewMode: {}", data.view_name)
    });

    let label_processes =
        Label::new(|data: &AppState, _env: &_| data.process_info.clone()).with_text_size(16.0);
    let scrollable_label_processes = Scroll::new(label_processes);

    let sized_scroll = SizedBox::new(scrollable_label_processes)
    .expand_height()
    .height(510.0)
    .expand_width()
    .width(400.0) // Wrap the Label in a Scroll widget to make it scrollable
    .border(Color::GRAY, 3.5)
    //.expand()
    ;

    let label_global_cpu: Label<AppState> = Label::new(|data: &AppState, _env: &_| {
        // Fix: Use the name field from the AppState.
        data.global_cpu.to_string()
    })
    .with_text_size(16.0);

    let label_global_mem: Label<AppState> = Label::new(|data: &AppState, _env: &_| {
        // Fix: Use the name field from the AppState.
        data.global_mem.to_string()
    })
    .with_text_size(16.0);

    let check_pname = Checkbox::new("Process Name")
        .lens(AppState::view_pname)
        .on_click(|_ctx, _data, _env| {
            // Handle checkbox click event
            _data.view_pname = !_data.view_pname;
            _data.process_info = printproc(
                !_data.view_option,
                _data.view_pname,
                _data.view_cpu,
                _data.view_mem,
                _data.view_usr,
                _data.view_path,
            )
            .unwrap_or("".to_string());
            //println!("Checkbox clicked: {}",_data.view_pname);
        });

    let check_cpu = Checkbox::new("Process Cpu")
        .lens(AppState::view_cpu)
        .on_click(|_ctx, _data, _env| {
            // Handle checkbox click event
            _data.view_cpu = !_data.view_cpu;
            _data.process_info = printproc(
                !_data.view_option,
                _data.view_pname,
                _data.view_cpu,
                _data.view_mem,
                _data.view_usr,
                _data.view_path,
            )
            .unwrap_or("".to_string());
            //println!("Checkbox clicked: {}",_data.view_cpu);
        });

    let check_mem = Checkbox::new("Process Mem")
        .lens(AppState::view_mem)
        .on_click(|_ctx, _data, _env| {
            // Handle checkbox click event
            _data.view_mem = !_data.view_mem;
            _data.process_info = printproc(
                !_data.view_option,
                _data.view_pname,
                _data.view_cpu,
                _data.view_mem,
                _data.view_usr,
                _data.view_path,
            )
            .unwrap_or("".to_string());
            //println!("Checkbox clicked: {}",_data.view_mem);
        });

    let check_usr = Checkbox::new("Process User")
        .lens(AppState::view_usr)
        .on_click(|_ctx, _data, _env| {
            // Handle checkbox click event
            _data.view_usr = !_data.view_usr;
            _data.process_info = printproc(
                !_data.view_option,
                _data.view_pname,
                _data.view_cpu,
                _data.view_mem,
                _data.view_usr,
                _data.view_path,
            )
            .unwrap_or("".to_string());
            //println!("Checkbox clicked: {}",_data.view_usr);
        });
    let check_path = Checkbox::new("Process Path")
        .lens(AppState::view_path)
        .on_click(|_ctx, _data, _env| {
            // Handle checkbox click event
            _data.view_path = !_data.view_path;
            _data.process_info = printproc(
                !_data.view_option,
                _data.view_pname,
                _data.view_cpu,
                _data.view_mem,
                _data.view_usr,
                _data.view_path,
            )
            .unwrap_or("".to_string());
            //println!("Checkbox clicked: {}",_data.view_path);
        });

    let button = Button::new("Change ViewMode")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.view_option = !data.view_option;
            if data.view_option {
                data.view_name = "Tree".to_string();
                data.process_info = printproc(
                    !data.view_option,
                    data.view_pname,
                    data.view_cpu,
                    data.view_mem,
                    data.view_usr,
                    data.view_path,
                )
                .unwrap_or("".to_string());
            } else {
                data.view_name = "List".to_string();
                data.process_info = printproc(
                    !data.view_option,
                    data.view_pname,
                    data.view_cpu,
                    data.view_mem,
                    data.view_usr,
                    data.view_path,
                )
                .unwrap_or("".to_string());
            }
            data.global_cpu = printglobalcpu().unwrap_or("".to_string());
            data.global_mem = printglobalmem().unwrap_or("".to_string());
        })
        .fix_width(200.0)
        .fix_height(50.0);

    let checkbox_layout = Flex::column()
        .with_child(check_pname)
        .with_spacer(0.5 * VERTICAL_WIDGET_SPACING)
        .with_child(check_cpu)
        .with_spacer(0.5 * VERTICAL_WIDGET_SPACING)
        .with_child(check_mem)
        .with_spacer(0.5 * VERTICAL_WIDGET_SPACING)
        .with_child(check_usr)
        .with_spacer(0.5 * VERTICAL_WIDGET_SPACING)
        .with_child(check_path)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);

    let global_layout = Flex::column()
        .with_child(label_global_cpu)
        .with_spacer(1.0 * VERTICAL_WIDGET_SPACING)
        .with_child(label_global_mem)
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        //.fix_width(600.0)
        .border(Color::GRAY, 3.5)
        .expand();

    // arrange the two widgets vertically, with some padding
    let change_view_layout = Flex::column()
        .with_child(label_view_mode)
        .with_spacer(1.0 * VERTICAL_WIDGET_SPACING)
        .with_child(button)
        .with_spacer(1.0 * VERTICAL_WIDGET_SPACING)
        .with_child(checkbox_layout)
        .with_spacer(10.0 * VERTICAL_WIDGET_SPACING + 12.5)
        .with_child(global_layout)
        .fix_size(340.0, 600.0)
        .align_horizontal(UnitPoint::CENTER);

    let layout2 = Flex::row()
        .with_spacer(10.0)
        .with_child(sized_scroll)
        .with_flex_spacer(50.0)
        .with_child(change_view_layout)
        .with_spacer(25.0);

    // center the two widgets in the available space
    Align::centered(layout2)
}
fn main() {
    //K = PID, V = LIST OF SUBPROCESSES
    //let mut _process_list : HashMap<u32, Vec<ProcessData>> = HashMap::new();

    // printproc().unwrap();

    // printglobal().unwrap();

    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((850.0, 600.0));

    // create the initial app state
    let initial_state = AppState {
        view_name: "not selected yet".into(),
        view_option: false,
        view_pname: false,
        view_cpu: false,
        view_mem: false,
        view_usr: false,
        view_path: false,
        process_info: "".into(),
        global_cpu: "".into(),
        global_mem: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
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

fn printproc(
    mode: bool,
    pname: bool,
    cpu: bool,
    mem: bool,
    user: bool,
    path: bool,
) -> Result<String, io::Error> {
    let mut processes: Vec<ProcessData> = Vec::new();

    let mut _process_list: HashMap<u32, Vec<u32>> = HashMap::new();

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

                let mut add: ProcessData =
                    ProcessData::new(0, 0, "".to_string(), 0.0, 0, "".to_string(), "".to_string());
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
                            let system_uptime: &&str = status_args.first().unwrap();

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
                                            //let proc_usage_procents = proc_usage_procents as u32;

                                            //println!("CPU usage: {}%", proc_usage_procents);
                                            add.cpu_used =
                                                (proc_usage_procents * 100.0).round() / 100.0;
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

    for process in &processes {
        let mut children: Vec<u32> = Vec::new();
        for ischild in &processes {
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
    let mut output: String = String::new();
    // for (key, value) in sorted_vec {

    //     println!("PID: {}, SUB-PIDS: {:?}", key, value);
    // }

    //println!("{:?}", _process_list);
    processes.sort_by_key(|proc| proc.pid);

    if mode {
        for proc in &processes {
            if pname {
                output.push_str("Name: ");
                output.push_str(&proc.name);
                output.push_str(" \n");
            }
            if cpu {
                output.push_str("Cpu used: ");
                output.push_str(&proc.cpu_used.to_string());
                output.push_str("%\n");
            }
            if mem {
                output.push_str("Mem used: ");
                output.push_str(&proc.memory_used.to_string());
                output.push_str(" Kb\n");
            }
            if path {
                output.push_str("Path: ");
                if proc.path.is_empty() {
                    output.push_str(".\n");
                } else {
                    output.push_str(&proc.path);
                    output.push_str(" \n");
                }
            }
            if user {
                output.push_str("Username: ");
                output.push_str(&proc.author);
                output.push_str(" \n");
            }
            // output.push_str("\nPid: ");
            // output.push_str(&proc.pid.to_string());
            output.push_str(" \n");
        }
    } else {
        let mut out: String = String::new();
        display_tree(
            1,
            &sorted_vec,
            1,
            &mut out,
            &processes,
            pname,
            cpu,
            mem,
            user,
            path,
        );
        output = out;
    }

    Ok(output)
}
#[allow(clippy::too_many_arguments)]
fn display_tree(
    pid: u32,
    process_list: &Vec<(u32, Vec<u32>)>,
    level: usize,
    ans: &mut String,
    process: &Vec<ProcessData>,
    pname: bool,
    cpu: bool,
    mem: bool,
    user: bool,
    path: bool,
) {
    for _ in 1..level {
        //print!("  ");
        //ans.push_str("    ");
        ans.push_str("----");
    }

    if let Some((current_pid, children)) = process_list.iter().find(|(id, _)| *id == pid) {
        //println!("PID {}", current_pid);
        ans.push('[');
        ans.push_str(&current_pid.to_string());
        ans.push(']');
        for proc in process {
            if proc.pid == *current_pid {
                if pname {
                    ans.push('[');
                    ans.push_str(&proc.name);
                    ans.push(']');
                }
                if cpu {
                    ans.push('[');
                    ans.push_str(&proc.cpu_used.to_string());
                    ans.push_str("%]");
                }
                if mem {
                    ans.push('[');
                    ans.push_str(&proc.memory_used.to_string());
                    ans.push_str(" Kb]");
                }
                if user {
                    ans.push('[');
                    ans.push_str(&proc.author);
                    ans.push(']');
                }
                if path {
                    ans.push('[');
                    ans.push_str(&proc.path);
                    ans.push(']');
                }
                break;
            }
        }
        ans.push_str(" \n");
        for &child_pid in children {
            display_tree(
                child_pid,
                process_list,
                level + 1,
                ans,
                process,
                pname,
                cpu,
                mem,
                user,
                path,
            );
        }
    }
}

fn printglobalcpu() -> Result<String, io::Error> {
    let mut out: String = String::from("GLOBAL CPU USAGE: ");

    let mut cpu_usage: f64 = 0.0;
    let paths: fs::ReadDir = fs::read_dir("/proc").unwrap();
    for path in paths {
        let file_name: std::ffi::OsString = path.unwrap().file_name();
        if let Some(name) = file_name.to_str() {
            if name.parse::<f64>().is_ok() {
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
                            let system_uptime: &&str = status_args.first().unwrap();

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
                                            //let proc_usage_procents = proc_usage_procents as u32;

                                            //println!("CPU usage: {}%", proc_usage_procents);
                                            let cpu_used =
                                                (proc_usage_procents * 100.0).round() / 100.0;
                                            cpu_usage += cpu_used.floor();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let cpu_usage: f64 = ((cpu_usage * 100.0).round() / 100.0) % 100.0;
    out.push_str(&cpu_usage.to_string());
    out.push('%');

    Ok(out)
}

fn printglobalmem() -> Result<String, io::Error> {
    let global_proc_stat_path: String = String::from("/proc/meminfo");
    let mem_stat: String = fs::read_to_string(global_proc_stat_path)?;
    let mut mem_total_string = String::new();
    let mut mem_available_string = String::new();

    for line in mem_stat.lines() {
        if line.starts_with("MemTotal") {
            let args: Vec<&str> = line.split_whitespace().collect();
            mem_total_string = args.get(1).unwrap().to_string();
        }
        if line.starts_with("MemAvailable") {
            let args: Vec<&str> = line.split_whitespace().collect();
            mem_available_string = args.get(1).unwrap().to_string();
        }
    }
    let mem_total_u32 = mem_total_string.parse::<u32>().unwrap();
    let mem_available_u32 = mem_available_string.parse::<u32>().unwrap();

    let mem_totgb: f64 = ((mem_total_u32 as f64 * 0.000001) * 100.0).round() / 100.0;

    let global_usage = mem_total_u32 - mem_available_u32;
    let global_gb_usage: f64 = ((global_usage as f64 * 0.000001) * 100.0).round() / 100.0;

    // println!("CPU USAGE: {}%",global_usage);

    let mut out = String::from("GLOBAL MEM USAGE: ");
    out.push_str(&global_gb_usage.to_string());
    out.push_str("GB / ");
    out.push_str(&mem_totgb.to_string());
    out.push_str("GB");
    Ok(out)
}
