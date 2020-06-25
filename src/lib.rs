use serde::Deserialize;
use reqwest::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;


fn get_api_key() -> io::Result<String> {
    
    let mut file = File::open(".xivapikey")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?; 

    Ok(token)
}

#[derive(Deserialize, Debug)]
struct SearchResults {
    Results: Vec<CharacterData>,
}
#[derive(Deserialize, Debug)]
struct CharacterData {
    ID: u64,
}

#[tokio::main]
async fn get_character_id(name: &str, server: &str) -> Result<u64, Error> {
    let api_key = get_api_key().expect("Problem getting key");
    let request_url = format!("https://xivapi.com/character/search?name={name}&server={server}&page=1",
                                        name = name,
                                        server = server
                                      );
    
    println!("Fetching {}...", request_url);
    
    let response = reqwest::get(&request_url).await?;
    let mut results: SearchResults = response.json().await?;
    let character_data = results.Results.pop().expect("Character could not be found");
    
    println!("Your character's ID is {}.", character_data.ID.to_string());
                        
    Ok(character_data.ID)

}

#[test]
fn char_data_test() {
    assert_eq!(get_character_id("Aika Shibuya", "Midgardsormr").unwrap(),1753092);
}
