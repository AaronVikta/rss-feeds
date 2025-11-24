use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "rss_cli")]
struct Cli {
    //The RSS feed URL
    #[command(subcommand)]
    command: Actions,
}


#[derive(Debug,Subcommand)]
enum Actions{

    Add{
    #[arg(short, long)]
    url: String
    },

    //Fetch 5 RSS feeds from the given URL
    Fetch{
    #[arg(short, long)]
    url: String,
    },
    Read{
    #[arg(short, long)]
    id: usize
    }
}

fn main() {
    println!("A CLI based RSS FEED");
}

