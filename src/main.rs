// use std::env;
// use std::fs;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::*;

use serde::Serialize; // 1.0.91
use toml; // 0.5.1

fn write_toml(file: &mut Config, path: &std::path::PathBuf) {
    let toml_string = toml::to_string(&file).expect("Could not encode TOML value");
    println!("{}", toml_string);
    fs::write(path, toml_string).expect("Could not write to file!");
}

#[derive(Deserialize, Default, Serialize)]
struct Config {
    toml: TomlConfig,
}

#[derive(Deserialize, Default, Serialize)]
struct TomlConfig {
    in_path: String,
    out_path: String,
}

fn main() {
    let default_config = include_str!("config.toml");

    let exe_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("config.toml");

    if exe_path.exists() {
        let mut config: Config =
            toml::from_str(&fs::read_to_string(&exe_path).expect("string")).expect("parse");
            match_args(&mut config);
    } else {
        let mut config: Config = toml::from_str(&default_config).expect("default failed to parse");
        write_toml(&mut config, &exe_path);
        match_args(&mut config);
    }


}

fn match_args(config: &mut Config) {
    let mode = env::args().nth(1).unwrap_or_default();
    let flag = env::args().nth(2).unwrap_or_default();
    let path = env::args().nth(3).unwrap_or_default();

    match mode.as_str() {
        "help" | "save me" => {
            println!("help mode ");
            match flag.as_str() {
                "america" => println!("america"),
                _ => println!("not america",),
            }
        }
        "wife" => {
            println!("wife mode ");
            match flag.as_str() {
                "america" => println!("america"),
                _ => println!("not america",),
            }
        }
        "save" => {
            println!("save mode ");
            match flag.as_str() {
                "" => error_path(),
                _ => update_path(config, path),
            }
        }
        "" => {
            println!("no mode supplied");
            match flag.as_str() {
                "america" => println!("america"),
                _ => println!("not america",),  
            }
        }
        _ => {
            println!("unrecognized mode");
            match flag.as_str() {
                "america" => println!("america"),
                _ => println!("not america",),
            }
        }
    }
}

fn error_path(){
    println!("path not provided") ;
}
// self, self, self
fn update_path(config: &mut Config, path: String){
    config.toml.in_path = config.toml.in_path.replace(&config.toml.in_path, &path);
}