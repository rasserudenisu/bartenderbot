extern crate serenity;
use serenity::model::{Message};

pub fn get_help(message: &serenity::model::Message) {
    let mut command: String = message.content.replace("!help", "");
    let mut response: String = "".to_string();
    match command.trim() {
        "drink" => response = format!("{}: {}", "!drink", "Buy a drink").to_string(),
        "sandwich" => response = format!("{}: {}", "!sandwich", "Buy a sandwich").to_string(),
        "roll" => response = format!("{}: {} -- Example: {}", "!roll", "Rolls up to 3d20. Defaults to 1d6.", "!roll 2d10").to_string(),
        "coin" => response = format!("{}: {}", "!coin", "Flips a coin").to_string(),
        "hiragana" => response = format!("{}: {} -- Example: {}", "!hiragana", "Converts romaji to hiragana. Should be well formatted.", "!hiragana nekoninaritai").to_string(),
        "romaji" => response = format!("{}: {} -- Example: {}", "!romaji", "Converts hiragana to romaji. Should be well formatted.", "!romaji ねこになりたい").to_string(),
        _ => response = "Available commands: !sandwich, !drink, !roll, !coin, !romaji, !hiragana, !slap. Type *!help <command>* for more information.".to_string()
    };
    message.reply(&response);
}
