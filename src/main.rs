use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use csv::Writer;
use ring::digest;
use std::str;
use std::io::{Write};

/**
 *  This is a URL Shortner project, making it very simple for first iteration
 * Will use redis and deploy to the web soon, this is to learn of what Rust is all about :) 
 *  It will be written on one page, here is the plan 
 * 1.) Take user input such as url link to a website 
 * 2.) Using one of the hashing algorithms, will convert the url into a hash 
 * 3.) Provide that hash to the client, create a <key, value> pain indicating something linke <hash, orginal link> in a .txt file or database 
 * 4.) When user provides the link to the shortened url, will do a key value search and redirect the user to this site  
 * 
*/ 

fn store_key_val_pairs(key: &str, val: &str) -> Result<(), Box<dyn Error>> {
   let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("database.csv")?;
    write!(file, "{},{}", key, val)?;
    Ok(())  
}
fn read_key_val_pairs(){
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        println!("{:?}", record);
    }
}

fn redirect(value : &str)
{

}

fn create_and_prompt()
{
    println!("Welcome to the URL shortner service (written in RUST), please type in a url below to get started ");
    let mut url  = String::new();
    io::stdin().read_line(&mut url).expect("failed to readline");
    let digest = digest::digest(&digest::SHA256, url.as_bytes());
    println!("You typed {} ", url); 
    let hex_string = hex::encode(digest);
    let first_5_chars = &hex_string[..5];
    let final_url = format!("Salzeem.ly/{}", first_5_chars); 
    println!("Your hash has been successfuly stored! Here is your link, {}", final_url ); 
    let val = store_key_val_pairs(&first_5_chars, &url);
    match val {
            Ok(()) => {
                println!("Storing has been succesfull! ");
            }
            Err(err) => {
                println!("Hmm, something went wrong");
            }
        }

}

fn main() {

    loop {
        println!("Welcome to the URL shortner service (written in RUST), type 1 to shorten a url, type 2 to get a hashed url, type 3 to exit");
        let mut option = String::new(); 
        io::stdin().read_line(&mut option).expect("Failed to read the line"); 
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if (option == 1)
        {
            create_and_prompt(); 

        }
        
        else if (option ==2 )
        {
            println!("This would be the retrieval step of the url");
        }

        else
        {
            println!("Goodbye fellow Rustaecean");
            break
        }


    }   
    


        
    

}


