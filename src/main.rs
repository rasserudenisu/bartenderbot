#[macro_use]
extern crate serenity;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use serenity::client::Context;
use serenity::Client;
use serenity::model::{Message, permissions, Game};
use serenity::ext::framework::help_commands;

use std::env;

use dotenv::dotenv;

mod commands;

fn main() {

    dotenv().ok();
    let token = env::var("BOT_TOKEN")
    .expect("Expected a token in the environment");

    let mut client = Client::login_bot(&token);
    client.with_framework(|f| f
        .configure(|c| c.prefix("!"))
        .on("slap", slap)
        .on("roll", roll)
        .on("coin", coin)
        .on("sandwich", sandwich)
        .on("drink", drink)
        .on("help", help)
        .on("romaji", romaji)
        .on("hiragana", hiragana)
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    client.on_ready(|context, ready| {
        println!("{} is connected!", ready.user.name);
    });
}

command!(slap(context, message) {
    let _ = message.reply("Oshioki Kibonnu!!");
});

command!(romaji(context, message) {
    let _ = message.reply(commands::jputils::to_romaji(message.content.as_str()).as_str());
});

command!(hiragana(context, message) {
    let _ = message.reply(commands::jputils::from_romaji(message.content.as_str()).as_str());
});

command!(help(context, message) {
    commands::help::get_help(message);
});

command!(sandwich(context, message) {
    let _ = context.say(commands::sandwich::generate_sandwich(message.author.name.as_str()).as_str());
});

command!(drink(context, message) {
    let _ = message.reply(commands::brewer::generate_drink(message.author.name.as_str()).as_str());
});

command!(roll(context, message) {
    let _ = message.reply(commands::roll::roll(message).as_str());
});

command!(coin(context, message) {
    let _ = message.reply(commands::roll::coin());
});
