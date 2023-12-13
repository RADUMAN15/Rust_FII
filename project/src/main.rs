use tasklist;
use tasklist::info;
use tasklist::infos::CpuTime;
fn main(){

    unsafe{
        //println!("open the debug priv{:?}",tasklist::enable_debug_priv());
        let tasklist: tasklist::Tasklist = tasklist::Tasklist::new();

        let mut ct = 0;
        // Iterate over processes and print information
        for process in tasklist {
            
            println!("Process Name: {}", process.get_pname());

            let cpu_data : (String,String,CpuTime) =  tasklist::get_proc_time(process.get_pid());            
            let user_time = cpu_data.2.get_user_time();
            print!("CPU used: ");
            for (i,col) in user_time.split(":").enumerate(){
                match i {
                    0 => print!("{} hours  ", col),
                    1 => print!("{} minutes  ", col),
                    2 => print!("{} seconds", col),
                    _ => println!("Unexpected index {}: {}", i, col),
                }
            }
            println!("");

            let mem_data = info::get_proc_memory_info(process.get_pid());
            println!("Process Memory: {:?}K",mem_data.get_quota_peak_non_paged_pool_usage());

            println!("Path File : {}", process.get_path());

            println!("User Name: {}", process.get_user());

            println!("");

            ct +=1;
        }
        println!("Size of tasklist is : {}",ct);
    }
}
