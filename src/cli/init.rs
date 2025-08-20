use std::io::Write;
use std::path::{PathBuf, Path};
use std::process::Command;

pub fn run(){
    match create_proj_folder(){
        Ok(_) => println!("check: sucessfully set up faproj folder"),
        Err(e) => eprintln!("check: fail to set up faproj folder due to {}", e),
    }
    check_r_py();
    println!("✓ fa2 init at USER_FA_DIR={}", std::env::var("USER_FA_DIR").unwrap());
}

fn check_r_py() {
    println!("check: install R, Ptyhon, and Quarto");
    Command::new("R")
        .arg("--version")
        .output()
        .expect("✘ R not installed");
    Command::new("python3")
        .arg("--version")
        .output()
        .expect("✘ Python not installed");
    Command::new("quarto")
        .arg("--version")
        .output()
        .expect("✘ Quarto not installed");
}

fn create_proj_folder() -> anyhow::Result<()>{
    // USER_FA_DIR="/home/stproj/testf" + faproj
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_proj_conf = path_root.join("faproj/config.toml");
    let path_box = path_root.join("faproj/box/stbox");
    let path_proj = path_root.join("faproj/job");
    // do not init again
    if path_proj_conf.exists(){
        eprintln!("✘ do not init again");
    }else{
        // create faproj dir
        std::fs::File::create(path_proj_conf)?;
        std::fs::create_dir_all(path_box)?;
        std::fs::create_dir_all(path_proj)?;
        // write to box.R
        let path_box_r = path_root.join("faproj/box/stbox/box.R");
        copy_box(&path_box_r)?;
        // write to R config.yaml
        let path_stbox = path_root.join("faproj/box");
        let path_box_yml = path_root.join("faproj/box/config.yml");
        write_r_yml(&path_stbox, &path_box_yml)?;
    }
    Ok(())
}

fn copy_box(des: &Path) -> anyhow::Result<()> {
    std::fs::File::create(des)?;
    let ctx = include_str!("../temp/box.R");
    std::fs::write(des, ctx)?;
    Ok(())
}

fn write_r_yml(path_stbox: &Path, path_box_yml: &Path) -> anyhow::Result<()>{
    let ctx = format!("default:\n  rbox: {}", path_stbox.to_str().unwrap().to_string());
    let mut file_yml = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_box_yml)?;
    file_yml.write(ctx.as_bytes())?;
    Ok(())
}

