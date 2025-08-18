use clap::Parser;

use fa2::Cli;

fn main() {
    let cli = Cli::parse();

    if !cli.home.exists(){
        eprintln!("set fa2 home by cmd init");
    }else{
        cli.cmd.run();
    }
}
