use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod init;
mod new;

use crate::cli::new::Job;

/// Financial Audit Tool © CA Analytics
#[derive(Parser)]
#[command(version, about)]
pub struct Cli{
    /// fa2 home dir
    #[arg(long, hide(true), env = "USER_FA_DIR")]
    pub home: PathBuf,
    #[command(subcommand)]
    pub cmd: FCommand, 
}

#[derive(Subcommand)]
pub enum FCommand{
    /// Init fa2 home dir and faudit.toml
    Init{
        /// fa2 home dir
        #[arg(short, long, default_value = ".")]
        root: PathBuf, 
    }, 
    /// Create a new audit job
    New(Job), 
    /// Generate quarto docs
    Report, 
    /// Run a web server
    Serve, 
    /// Reconcile financial data
    Check, 
    /// List a job folder structure
    Show, 
}

impl FCommand{
    pub fn run(&self){
        match self{
            Self::Init { root } => init::run(root),
            Self::New( job ) => job.run(),
            Self::Report => println!("report fa2"),
            Self::Serve => println!("run axum web server"),
            Self::Check => println!("check financial numbers"),
            Self::Show => println!("list fa2 folder structure"),
        }
    }
}

