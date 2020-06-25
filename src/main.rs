use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
};
#[group]
#[commands(hello_world)]
struct General;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Handler;

impl EventHandler for Handler {}


fn main() {
   
    let mut token = get_token().expect("Error getting token");
    let mut client = Client::new(token, Handler).expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
        }
}

fn get_token() -> io::Result<String> {
    
    let mut file = File::open(".token")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?; 

    Ok(token)
}

#[command]
fn hello_world(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello World!")?;

    Ok(())
}