use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod init;
mod new;
mod report;
mod serve;
mod show;

use crate::cli::new::Job;

/// Financial Audit Tool © CA Analytics
#[derive(Parser)]
#[command(version, about)]
pub struct Cli{
    #[arg(long, hide(true), env = "USER_FA_DIR")]
    pub home: PathBuf,
    #[command(subcommand)]
    pub cmd: FCommand, 
}

#[derive(Subcommand)]
pub enum FCommand{
    /// Init fa2 config.toml
    Init, 
    /// Create a new job
    New(Job), 
    /// Generate quarto docs
    Report{
        /// Query job client name
        #[arg(short, long)]
        client: String, 
        /// Query job client year
        #[arg(short, long)]
        year: String, 
        /// Quarto other args
        #[arg(short, long)]
        qargs: Vec<String>, 
    }, 
    /// Run a web server
    Serve{
        /// IP addr serve
        #[arg(short, long, default_value = "127.0.0.1:8090")]
        addr: String,
    }, 
    /// Reconcile financial data
    Check, 
    /// List a job folder structure
    Show, 
}

impl FCommand{
    pub fn run(self){
        match self{
            Self::Init => init::run(),
            Self::New( job ) => new::run(job),
            Self::Report { client, year, qargs } => report::run(&client, &year, &qargs),
            Self::Serve { addr } => serve::run(&addr),
            Self::Check => println!("check financial numbers"),
            Self::Show => show::run(), 
        }
    }
}

