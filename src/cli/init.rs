use std::io::Write;
use std::path::{PathBuf, Path};
use std::process::Command;

pub fn run(root: &PathBuf){
    // bash export USER_FA_DIR="/home/stli/stproj/tryrs/testf"
    println!("Check if USER_FA_DIR={} is set", std::env::var("USER_FA_DIR").unwrap());
    check_r_py();
    create_proj_folder().expect("fail to set up proj folder");
    println!("✓ faproj init at {}", root.display());
}

fn check_r_py() {
    println!("Check if R, Ptyhon, and Quarto are installed?");
    Command::new("R")
        .arg("--version")
        .output()
        .expect("✘ R is not installed");
    Command::new("python3")
        .arg("--version")
        .output()
        .expect("✘ Python is not installed");
    Command::new("quarto")
        .arg("--version")
        .output()
        .expect("✘ Quarto is not installed");
}

fn create_proj_folder() -> anyhow::Result<()>{
    let dir1 = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let dir2 = std::env::current_dir().unwrap();
    assert_eq!(dir1, dir2, "cd fa2 root to init");
    // create faproj dir
    let path_proj = dir2.join("faproj/job");
    let path_box = dir2.join("faproj/box/stbox");
    let path_proj_conf = dir2.join("faproj/config.toml");
    std::fs::create_dir_all(path_proj)?;
    std::fs::create_dir_all(path_box)?;
    std::fs::File::create(path_proj_conf)?;
    // write content to box
    let path_box_r = dir2.join("faproj/box/stbox/box.R");
    copy_box(&path_box_r)?;
    // write content to yaml
    let path_stbox = dir2.join("faproj/box");
    let path_box_yml = dir2.join("faproj/box/config.yml");
    write_r_yml(&path_stbox, &path_box_yml)?;
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
    file_yml.write(ctx.as_bytes()).unwrap();
    Ok(())
}
