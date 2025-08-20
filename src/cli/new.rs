use std::path::{PathBuf, Path};
use clap::Args;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Serialize, Deserialize)]
struct Config{
    jobs: Vec<Job>,
}

#[derive(Args, Serialize, Deserialize)]
pub struct Job{
    /// Client name
    #[arg(short, long)]
    pub client: String, 
    /// Financial year
    #[arg(short, long)]
    pub year: String, 
    /// PBC documents
    #[arg(value_parser(clap::value_parser!(PathBuf)))]
    pbc: Vec<PathBuf>, 
}

pub fn run(job: Job) {
    // to lowercase
    let Job { client, year, pbc } = &job;
    let client = client.to_lowercase();
    // job/client [same name]/year [2024, 2025]
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_proj_conf = path_root.join("faproj/config.toml");
    let path_year = path_root.join("faproj/job").join(client).join(year);
    match path_year.exists(){
        true => eprintln!("✘ client exists, cd {}", path_year.display()), 
        false => {
            std::fs::create_dir_all(&path_year).expect("fail to create client path");
            create_job_folder(&path_year, pbc).expect("fail to create client folder");
            println!("✓ client created at {}", path_year.display());
        }
    }
    // cd to job/client/year and copy clean.R and report.qmd
    std::env::set_current_dir(path_year).unwrap();
    copy_include().expect("fail to copy temp files to client");
    // write job to config.toml 
    load_config_toml(job, &path_proj_conf).expect("fail to modify config.toml");
}

fn create_job_folder(path_year: &Path, pbc_raw: &[PathBuf]) -> anyhow::Result<()> {
    let path_pbc = path_year.join("pbc");
    let path_awp = path_year.join("awp");
    let path_report = path_year.join("report");
    std::fs::create_dir_all(&path_pbc)?;
    std::fs::create_dir_all(path_awp)?;
    std::fs::create_dir_all(path_report)?;
    // move cli pbc args to pbc folder
    pbc_raw.iter().for_each(|x| {
        let fname = x.file_name().unwrap().to_str().unwrap();
        let des = path_pbc.join(fname);
        std::fs::rename(x, des).expect("fail to copy pbc raw files");
    });
    Ok(())
}

fn copy_include() -> anyhow::Result<()> {
    std::fs::File::create("pbc/clean.R")?;
    let ctx_clean = include_str!("../temp/clean.R");
    std::fs::write("pbc/clean.R", ctx_clean)?;
    // pbc and report folder
    std::fs::File::create("report/report.qmd")?;
    let ctx_report = include_str!("../temp/report.qmd");
    std::fs::write("report/report.qmd", ctx_report)?;
    Ok(())
}

fn load_config_toml(job: Job, path_proj_conf: &Path) -> anyhow::Result<()>{
    let ctx = std::fs::read_to_string(path_proj_conf)?;
    if let Ok(res) = ctx.parse::<Table>(){
        // if it is the first time
        if res.is_empty(){
            let config = Config{jobs: vec![job]};
            let ctx_out = toml::to_string_pretty(&config)?;
            std::fs::write(path_proj_conf, ctx_out)?;
        }else{
            let mut existing_jobs = toml::from_str::<Config>(&ctx)?;
            existing_jobs.jobs.push(job);
            let ctx_out = toml::to_string_pretty(&existing_jobs)?;
            std::fs::write(path_proj_conf, ctx_out)?;
        }
    }
    Ok(())
}

