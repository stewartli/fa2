use clap::Parser;

use fa2::Cli;

fn main() {
    let cli = Cli::parse();

    if !cli.home.exists(){
        eprintln!("export USER_FA_DIR [fa2 home] to the shell");
    }else{
        cli.cmd.run();
    }
}
