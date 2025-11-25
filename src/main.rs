use clap::{Parser, Subcommand};

pub mod rssfunc;
use crate::rssfunc::{add_feed, get_feed, read_feed, read_multiple_feeds};


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
    #[arg(short, long, help="RSS feed URL to add")]
    url: String
    },

    //Fetch 5 RSS feeds from the given URL
    Fetch{
    #[arg(short, long, help="RSS feed URL to fetch")]
    url: String,
    limit:usize
    },
    Read{
    #[arg(short, long)]
    id: usize
    },
    List{
        limit:usize
    }
}
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    let cli=Cli::parse();

    match cli.command{
        Actions::Add { url }=>{
            add_feed(&url).await?;
        }
        Actions::Fetch { url ,limit} =>{
            
            get_feed(&url,limit).await?;
        }
        Actions::Read { id }=>{
            read_feed(id).await?;
        }
        Actions::List { limit }=>{
            read_multiple_feeds(limit).await?;
        }
    }
    
    Ok(())
}











