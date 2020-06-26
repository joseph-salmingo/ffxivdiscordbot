use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
        Args,
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
};
use serenity::http::AttachmentType;

#[group]
#[commands(hello_world, fyc)]
struct General;

use std::{env, path::Path};
use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Handler;

impl EventHandler for Handler {}


fn main() {
   
    let token = get_token().expect("Error getting token");
    let mut client = Client::new(token, Handler).expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
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

    // Replies to the command "!hello_world" with a generic "Hello World!"
    msg.reply(ctx, "Hello World!")?;

    Ok(())
}

#[command]
fn fyc(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    
    // Replies to the command "!fyc" with the following:
    // If a name is provided, adds a description to the embedded text to include the name. 
    // Otherwise, no description is used in the embedded message.
    // It then posts the Rick James Chapelle show image of him saying "Fuck yo' couch!"
    
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        
        let mut description_text = String::new();
        let arg_user = args.single::<String>();
        match arg_user {
            Ok(cmd_name) => description_text = format!("Hey {}...", cmd_name),
            _ => ()
        }
        m.add_file(AttachmentType::Path(Path::new("fyc.jpg")));
        m.embed(|e |  {
            e.title("Fuck yo' couch!");
            e.description(description_text);
            e.image("attachment://fyc.jpg");
            e
        });
        m
    }) {
        println!("Error sending embed: {}", why);
    }

    Ok(())
}