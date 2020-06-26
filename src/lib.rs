use serde::Deserialize;
use reqwest::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::error;


fn get_api_key() -> io::Result<String> {
    
    let mut file = File::open(".xivapikey")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?; 

    Ok(token)
}

#[derive(Deserialize, Debug)]
struct SearchResults {
    #[serde(rename = "Results")]
    results: Vec<CharacterSearchData>,
}
#[derive(Deserialize, Debug)]
struct CharacterSearchData {
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Deserialize, Debug)]
struct ProfileInformation {
    #[serde(rename = "Character")]
    character: CharacterProfileInformation
 }

#[derive(Deserialize, Debug, PartialEq, Default)]
 struct CharacterProfileInformation {

    #[serde(rename = "Avatar")]
    avatar: String,
    #[serde(rename = "DC")]
    dataCenter: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Portrait")]
    portrait: String,
    #[serde(rename = "Server")]
    server: String,
 }

#[tokio::main]
async fn get_character_id(name: &str, server: &str) -> Result<u64, Box<dyn error::Error>> {
    // Sends a request to the xiv lodestone API given the name and the server. 
    // The character ID is expected and if exactly one result is not given then it returns an error.

    let api_key = get_api_key().expect("Problem getting key");
    let request_url = format!("https://xivapi.com/character/search?name={name}&server={server}&page=1",
                                        name = name,
                                        server = server
                                      );
    
    println!("Fetching {}...", request_url);
    
    let response = reqwest::get(&request_url).await?;
    let mut results: SearchResults = response.json().await?;
    if results.results.len() != 1 {
        return Err("No Results".into())
    }
    let character_data = results.results.pop().expect("Character could not be found");
    
    Ok(character_data.id)

}

#[tokio::main]
async fn get_character_info(character_id: u64) -> Result<CharacterProfileInformation, Box<dyn error::Error>> {
    let api_key = get_api_key().expect("Problem getting key");
    let request_url = format!("https://xivapi.com/character/{character_id}",
                                        character_id = character_id
                                      );

    println!("Fetching {}...", request_url);

    let response = reqwest::get(&request_url).await?;
    let results: ProfileInformation = response.json().await?;

    Ok(results.character)

}

#[test]
fn char_data_test() {
    assert_eq!(get_character_id("Aika Shibuya", "Midgardsormr").unwrap(), 1753092);
}

#[test]
fn char_info_test() {
    
    let test_char =  CharacterProfileInformation {
        avatar: String::from("https://img2.finalfantasyxiv.com/f/475c634207855d204970fd88ed4c61ef_5c8ecfbc673e1287a9b5e85423fe1657fc0_96x96.jpg?1593183649"),
        dataCenter: String::from("Aether"),
        name: String::from("Aika Shibuya"),
        portrait: String::from("https://img2.finalfantasyxiv.com/f/475c634207855d204970fd88ed4c61ef_5c8ecfbc673e1287a9b5e85423fe1657fl0_640x873.jpg?1593183649"),
        server: String::from("Midgardsormr"),
    };
    
    let my_info: CharacterProfileInformation = get_character_info(1753092).unwrap();
    assert_eq!(test_char, my_info);
}
#[test]
fn complete_char_test() {
    let test_char =  CharacterProfileInformation {
        avatar: String::from("https://img2.finalfantasyxiv.com/f/475c634207855d204970fd88ed4c61ef_5c8ecfbc673e1287a9b5e85423fe1657fc0_96x96.jpg?1593183649"),
        dataCenter: String::from("Aether"),
        name: String::from("Aika Shibuya"),
        portrait: String::from("https://img2.finalfantasyxiv.com/f/475c634207855d204970fd88ed4c61ef_5c8ecfbc673e1287a9b5e85423fe1657fl0_640x873.jpg?1593183649"),
        server: String::from("Midgardsormr"),
    };

    let test_char_two =  CharacterProfileInformation {
        dataCenter: String::from("Aether"),
        name: String::from("Forte Rin"),
        server: String::from("Adamantoise"),
        ..Default::default()
    };

    let char_id = get_character_id("Aika Shibuya", "Midgardsormr").unwrap();
    let char_id_two: u64 = get_character_id("Forte Rin", "Adamantoise").unwrap();

    let my_info: CharacterProfileInformation = get_character_info(char_id).unwrap();
    let my_info_two: CharacterProfileInformation = get_character_info(char_id_two).unwrap();

    assert_eq!(test_char, my_info);
    assert_eq!(test_char_two.dataCenter, my_info_two.dataCenter);
}

