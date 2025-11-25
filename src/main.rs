use clap::{Parser, Subcommand};
use std::error::Error;

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
            
            get_feeds(&url,limit).await?;
        }
        Actions::Read { id }=>{
            println!("Reading RSS feed with ID: {}", id);
        }
    }
    
    Ok(())
}


async fn get_feeds(url: &str,limit:usize)-> Result<(),Box<dyn Error>>{
    //validate the URL  format
    if !url.starts_with("https://") && !url.starts_with("https://"){
        return Err("Invalid URL: must start with http:// or https://".into());
    }

    //Test if the URL is acceessible
    let response = reqwest::get(url).await?;

    if !response.status().is_success(){
        return  Err(format!("Failed to fetch RSS feed: HTTP {}", response.status()).into());
    }
    let content = response.text().await?;

    let channel = content.parse::<rss::Channel>()?;

    println!("Feed:{}", channel.title());
    println!("Description: {}", channel.description());
    println!("Items (showing up to {}):\n", limit);

    for (idx,item) in channel.items().iter().take(limit).enumerate(){

        println!("{}.{}", idx+1, item.title().expect("No title"));

        if let Some(desc) = item.description(){
            let clean_desc = desc.replace("<p>", "")
            .replace("</p>", "")
            .chars()
            .take(100)
            .collect::<String>();

        println!("{}{}", clean_desc,  if desc.len()>100{"..."} else{""});
        }
        if let Some(link) = item.link(){
            println!("Link: {}", link);
        }
        println!();
    }

    Ok(())
}


async fn add_feed(url:&str)->Result<(), Box<dyn Error>>{
    //validate  the  URL format
    if !url.starts_with("http://") && !url.starts_with("https://"){
        return Err("Invalid URL: must start with http:// or https://".into());
    } 
    //Test if the URL is accessible
    let response = reqwest::get(url).await?;

    if !response.status().is_success(){
        return Err(format!("Failed to access RSS feed: HTTP {}", response.status()).into());
    }

    println!("Successfully added RSS feed: {}", url);
    //Store URL in database or file #TODO
    Ok(())
}