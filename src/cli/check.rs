use std::process::Command;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use toml::Table;

pub fn run(){
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_proj_conf = path_root.join("faproj/config.toml");
    let ctx = std::fs::read_to_string(path_proj_conf).unwrap();
    check_config_toml(ctx);
}

fn check_config_toml(ctx: String){
    let tbl_toml = ctx.parse::<Table>().unwrap();
    if let Some(tbl_check) = tbl_toml.get("check"){
        if let Some(tbl_plugin) = tbl_check.get("plugin"){
            let path_plugin = PathBuf::from(tbl_plugin.as_str().unwrap());
            let ft = path_plugin.extension().and_then(OsStr::to_str).unwrap();
            if path_plugin.exists(){
                run_cmd(ft, &path_plugin);
            }else{
                eprintln!("✘ plugin file does not exist");
            }
        }
    }
}

fn run_cmd(ft: &str, path_plugin: &Path){
    match ft {
        "R" => plugin_cmd("R", path_plugin), 
        "py" => plugin_cmd("python3", path_plugin), 
        _ => eprintln!("✘ the program does not supported"),
    }
}

fn plugin_cmd(prog: &str, path_plugin: &Path){
    let cmd = Command::new(prog).arg(path_plugin).output().expect("fail to start the program");
    if cmd.status.success(){
        println!("✓ sucessfully run plugin");
    }else{
        eprintln!("✘ fail to run plugin");
    }
}
