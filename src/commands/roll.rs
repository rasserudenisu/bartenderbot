extern crate rand;
extern crate serenity;
use self::rand::thread_rng;
use self::rand::Rng;
use serenity::model::{Message};

pub fn coin() -> &'static str {
    let value: u8 = thread_rng().gen_range(0, 100);
    if value > 50 {
        return "flipped a coin. **Heads!**";
    }
    return "flipped a coin. **Tails!**";
}

pub fn roll(message: &serenity::model::Message) -> String {

    let mut true_dice: u8 = 1;
    let mut true_sides: u8 = 20;

    let mut content = message.content.replace("!roll", "");
    content = content.trim().to_string();

    if content.contains("d") {
        let index = content.find("d").unwrap();
        let contents = content.split_at(index);
        true_dice = parse_number(contents.0, 1, 3);
        let mut side_check = contents.1.to_string();
        side_check.remove(0);
        true_sides = parse_number(&side_check, 4, 20);
    }

    let mut results = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..true_dice {
        results.push(rng.gen_range(1, true_sides).to_string());
    }

    return format!("rolled {} dice: {}", true_dice, results.join(", "));
}

fn parse_number(target: &str, min: u8, max: u8) -> u8 {
    let mut value: u8 = 0;
    match target.parse::<u8>() {
      Ok(n) => value = n,
      Err(e) => println!("error {}", e),
    }
    if value > max {
        value = max;
    } else if value < min {
        value = min;
    }

    return value;
}
