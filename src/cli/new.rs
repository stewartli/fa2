use std::path::{PathBuf, Path};
use clap::Args;

#[derive(Args)]
pub struct Job{
    /// Client name
    #[arg(short, long)]
    pub client: String, 
    /// Financial year
    #[arg(short, long)]
    pub year: String, 
    /// PBC documents
    pub pbc: Vec<String>, 
}

impl Job{
    pub fn run(&self) {
        let path_proj = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
        let path_job = path_proj.join("job");
        let path_client = path_job.join(&self.client);
        let path_year = path_job.join(&self.client).join(&self.year);
        // check if job/client/year exists
        let goto_year = path_client.exists() && path_year.exists();
        match goto_year{
            true => eprintln!("✘ cd {} as it exists", &path_year.display()), 
            false => {
                std::fs::create_dir_all(&path_year).unwrap();
                create_job_folder(&path_year).expect("fail to create job folder");
                println!("✓ job created at {}", path_year.display());
            }
        }
        // cd to job/client/year and copy clean.R and report.qmd
        std::env::set_current_dir(&path_year).unwrap();
        copy_include().unwrap();
    }
}

fn create_job_folder(path_year: &Path) -> anyhow::Result<()> {
    let path_pbc = path_year.join("pbc");
    let path_awp = path_year.join("awp");
    let path_report = path_year.join("report");
    let path_doc = path_year.join("doc");
    std::fs::create_dir_all(&path_pbc)?;
    std::fs::create_dir_all(&path_awp)?;
    std::fs::create_dir_all(&path_report)?;
    std::fs::create_dir_all(&path_doc)?;
    Ok(())
}

fn copy_include() -> anyhow::Result<()> {
    std::fs::File::create("awp/clean.R")?;
    let ctx_clean = include_str!("../temp/clean.R");
    std::fs::write("awp/clean.R", ctx_clean)?;
    // awp and report folder
    std::fs::File::create("report/report.qmd")?;
    let ctx_report = include_str!("../temp/report.qmd");
    std::fs::write("report/report.qmd", ctx_report)?;
    Ok(())
}

