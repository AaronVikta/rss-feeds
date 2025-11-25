use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead,BufReader,Write};
use std::path::Path;

const FILE_PATH: &str = "feeds.txt";

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


async fn get_feed(url: &str,limit:usize)-> Result<(),Box<dyn Error>>{
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
    //Store feed in feeds.txt
    //check if file exists
    if Path::new(FILE_PATH).exists(){
        let file = File::open(FILE_PATH)?;
        let reader = BufReader::new(file);

        for line in reader.lines(){
            if let Ok(existing_url)= line{
                if existing_url.trim ()== url{
                    println!("Feed already exists in the list.");
                    return Ok(());
                }
            }
        }
    }

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(FILE_PATH)?;

    writeln!(file, "{}", url)?;
    println!("Successfully added RSS feed {}", url);

    Ok(())
}

async fn read_feed(id:usize)->Result<(), Box<dyn Error>>{

  if !Path::new(FILE_PATH).exists(){
    println!("No feeds found. Add feeds using cargo run add --url <URL>");

    return Ok(());
  }

  let file = File::open(FILE_PATH)?;
  let reader = BufReader::new(file);

  let urls: Vec<String> = reader.lines()
  .filter_map(|line|line.ok())
  .filter(|line|!line.trim().is_empty())
  .collect();

  //Collect all items from all feeds
  let mut all_items: Vec<(String, rss::Item)> = Vec::new();

  for url in urls{
    match reqwest::get(&url).await{
        Ok(response)=>{
            if let Ok(content)= response.text().await{
                if let Ok(channel)=content.parse::<rss::Channel>(){
                    for item in channel.items(){
                        all_items.push((url.clone(),item.clone()));
                    }
                }
            }
        }
        Err(_)=> continue,
    }
  }

  if id == 0|| id >all_items.len(){
    return Err(format!("Invalid ID. Please use ID between 1 and {}",all_items.len()).into());

  }
  let (feed_url,item) = &all_items[id -1];

  println!("\n =====================");
  println!("{}",item.title().expect("No title"));
  println!("=======================\n");

  if let Some(desc) = item.description(){
    println!("{} \n",strip_html_tags(desc));
  }

  if let Some(link) = item.link(){
    println!("Link: {}\n", link);
  }
  if let Some(pub_date) = item.pub_date(){
    println!("Published on: {}", pub_date);
  }
    println!("Source Feed: {}", feed_url);

    Ok(())
}


fn strip_html_tags(html:&str)->String{
    html.replace("<p>", "")
    .replace("</p>", "\n")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}


async fn read_multiple_feeds(limit:usize)->Result<(), Box<dyn Error>>{
    
    if !Path::new(FILE_PATH).exists(){
        println!("No feeds found. Add feeds using cargo run add --url <URL>");
        return Ok(());
    }

    let file = File::open(FILE_PATH)?;

    let reader = BufReader::new(file);

    let urls: Vec<String>= reader.lines()
    .filter_map(|line| line.ok())
    .filter(|line| !line.trim().is_empty())
    .collect();

    if urls.is_empty(){
        println!("No feeds found.Add feeds using cargo run add --url <URL>");
        return Ok(());
    }

    println!("Fetching {} feeds... \n",urls.len());

    for (idx, url) in urls.iter().enumerate(){
        println!("==============================");
        println!("Feed {}: {}", idx +1, url);
        println!("==============================\n");

        match get_feed(url, limit).await{
            Ok(_) =>{},
            Err(e)=>{
                println!("Error fetching feed{}\n", e);
            }
        }
    }
 
    Ok(())
}