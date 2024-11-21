use std::env;
use std::fs;
use std::io::{Read, Write};

fn help() {
    println!("Usage: autostart [options] [file]");
    println!("Options:");
    println!("    --h, print this help message and exit.");
    println!("    --getAutoStart, This will get auto start config from config path.");
    println!("    --configPath, Print config path of auto start hyprland.");
    println!("    --setConfigPath <path>, Set different path to config file.");
}

fn invalid_command(query: &str) {
    println!("{query} Command Not Found. Use --h for more.");
}

fn get_auto_start(home_path: &str) {
    let config_path = home_path.to_string() + "/.config/autostart/path.txt";
    let mut path = match fs::File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Could Not Found Path To Config File.");
            return;
        }
    };

    let mut path_file = String::new();

    path.read_to_string(&mut path_file).expect("Something went wrong reading the file");

    let mut file = match fs::File::open(&path_file) {
        Ok(file) => file,
        Err(_e) => {
            println!("Could Not Open File.Set Config Path From Running --setConfigPath And --h For Help.");
            return;
        }
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    contents.split('\n').for_each(|line| {
        if !line.trim_start().starts_with('#') && line.len() > 0 {
            if let Some(rest) = line.split_once("exec-once") {
                // Extract the part after the '='
                if let Some(command) = rest.1.split_once('=').map(|(_, cmd)| cmd.trim()) {
                    println!("{}", command);
                }
            }
        }
    })
}

fn config_path(home_path: &str) {
    let config_path = home_path.to_string() + "/.config/autostart/path.txt";
    let mut path = match fs::File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Could Not Found Path To Config File.Create new config path with --setConfigPath And --h For Help.");
            return;
        }
    };

    let mut path_file = String::new();

    path.read_to_string(&mut path_file).expect("Something went wrong reading the file");

    println!("{}", path_file);
}

fn set_config_path(home_path: &str ,new_config_path: &str) {
    let config_path = home_path.to_string() + "/.config/autostart/path.txt";
    let path = std::path::Path::new(&config_path).exists();

    if path {
        let mut path_to_config = match fs::File::open(&config_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Could Not Found Path To Config File.");
                return;
            }
        };

        match path_to_config.write_all(new_config_path.as_bytes()) {
            Ok(_) => println!("Successfully Set Config Path."),
            Err(_) => println!("Could not Set Config Path. Could Not Write To Exist Config File."),
        };
    }else {
        match fs::File::create(&config_path) {
            Ok(mut file) => {
                match file.write_all(new_config_path.as_bytes()) {
                    Ok(_) => println!("Successfully Set Config Path."),
                    Err(_) => println!("Could not Set Config Path. Could Not Write To Created Config File."),
                }
            }
            Err(e) => {
                println!("Could not Set Config Path. Could Not Create Config File.");
                println!("{e}");
                return;
            }
        };
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let binding = home::home_dir().unwrap();
    let home_path = binding.to_str().expect("Could not find home directory");

    if args.len() > 1 {
        let query = &args[1];

        match query.as_str() {
            "--h" => Ok(help()),
            "--getAutoStart" => Ok(get_auto_start(home_path)),
            "--configPath" => Ok(config_path(home_path)),
            "--setConfigPath" => {
                if args.len() > 2 {
                    Ok(set_config_path(home_path,&args[2]))
                } else {
                    Ok(println!("Missing config path argument. Use --h for more information."))
                }
            },
            _ => Ok(invalid_command(query))
        }
    } else {
        Ok(println!("No Args Provided. Type --h for help"))
    }
}
