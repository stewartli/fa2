use std::process::Command;
use std::path::{PathBuf, Path};

pub fn run(client: &str, year: &str, qargs: &[String]){
    // only support one report [client/year]
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_report = path_root.join("faproj/job")
        .join(client.to_lowercase())
        .join(year)
        .join("report/report.qmd");
    // run Quarto
    if path_report.exists(){
        run_qmd(&path_report, "render", qargs);
    }else{
        eprintln!("✘ client job report not found");
    }
}

fn run_qmd(path_report: &Path, act: &str, qargs: &[String]){
    let qmd = Command::new("quarto")
        .args([act, path_report.to_str().unwrap()])
        .args(qargs)
        .output()
        .expect("✘ fail to start quarto process");
    if qmd.status.success(){
        println!("✓ successfully run quarto {}", act);
    }else{
        eprintln!("✘ fail to run quarto {}", act);
    }
}

