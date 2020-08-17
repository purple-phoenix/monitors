use structopt::StructOpt;
use regex::{Regex, Error};
use std::{env, fs};
use sysinfo::{ProcessExt, System, SystemExt, Process, Signal};

fn main() {
    let monitor_config_dir = env!("MONITOR_CONFIG_DIR");
    println!("Looking for monitor configuration XMLs in {}. \
    You can change this by running make install again with an updated parameters.",
             monitor_config_dir);
    let args = Cli::from_args();
    let r = args.make_regex().unwrap();
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    let gnome_process = get_process_by_name(&system, "gnome-shell").unwrap();

    let home_dir = match dirs::home_dir() {
        Some(path) => {
            println!("Detected home directory: {}", path.display());
            path
        },
        None => {
            println!("Impossible to get your home dir!");
            std::process::exit(-1)
        },
    };

    let home_dir_str = home_dir.to_string_lossy();

    for entry in fs::read_dir(monitor_config_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let f_name = entry.file_name();
        let file_name = f_name.to_string_lossy();
        if file_name.ends_with(".xml") {
            if r.is_match(&file_name) {
                println!("First match found {}", file_name);
                let active_monitor_config_file = home_dir_str.clone() + "/.config/monitors.xml";
                println!("Copying {} to {}", file_name, active_monitor_config_file);
                match fs::copy(path, active_monitor_config_file.as_ref()) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Failed to copy!");
                        std::process::exit(-1)
                    }
                }
                for _ in 1..10 {
                    let is_killed = gnome_process.kill(Signal::Kill);
                    if is_killed {
                        return
                    }
                }

                println!("Failed to kill process:");
            }
        }

    }

}

fn get_process_by_name<'a>(system: &'a System, process_name: &'a str) -> Option<&'a Process>{
    for (_pid, process) in system.get_processes() {
        let p_name = process.name();
        if p_name == process_name {
            return Some(process)
        }
    }
    return None
}
#[derive(StructOpt)]
struct Cli {
    config_file_regex: String,

}


impl Cli {
    fn make_regex(&self) -> Result<Regex, Error> {
        return Regex::new(self.config_file_regex.as_ref())
    }
}