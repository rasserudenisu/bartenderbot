#![feature(proc_macro)]

extern crate serde_json;
extern crate rand;

use std::fs::File;
use self::rand::thread_rng;
use self::rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct Materials {
    beer: Vec<String>,
    mixers: Vec<String>,
    coffee: Vec<String>,
    spirits: Vec<String>,
    toppers: Vec<String>,
    glasses: Vec<String>
}

fn get_random_item(items: &Vec<String>) -> String {
    let index: usize = thread_rng().gen_range(0, &items.len()-1);
    return items.get(index).unwrap().clone();
}

pub fn generate_drink(author: &str) -> String {

    let file = File::open("data/drinks.json").unwrap();
    let materials: Materials = serde_json::from_reader(file).unwrap();

    let mut rng = thread_rng();
    let mut buffer: Vec<String> = Vec::new();
    buffer.push("ordered".to_string());
    let drinkType: u8 = rng.gen_range(0, 100);

    //get beer
    if drinkType >= 80 {
        buffer.push("a".to_string());
        buffer.push(get_random_item(&materials.beer));
        return buffer.join(" ");
    }
    //end beer - start neat / on ice
    else if drinkType < 80 && drinkType >= 70 {
        let onIce: u8 = rng.gen_range(0, 100);
        let drink = get_random_item(&materials.spirits);
        if onIce > 50 {//yes
            buffer.push(format!("{}, on ice.", drink));
        } else {//no
            buffer.push(format!("{}, straight.", drink));
        }
        return buffer.join(" ");
    }
    //end neat - start shot
    else if drinkType < 70 && drinkType > 60 {
        buffer.push("a shot of".to_string());
        buffer.push(get_random_item(&materials.spirits));
        return buffer.join(" ");
    }
    // end shot -- start cocktail
    else if drinkType < 60 && drinkType >= 5 {
        let beer_chance: u8 = rng.gen_range(0, 100);
        if beer_chance > 85 {
            buffer.push("a".to_string());
            buffer.push(get_random_item(&materials.beer));
        }
        else if beer_chance <= 85 {
            buffer.push(get_random_item(&materials.spirits));
        }

        //add commas later
        let addins: u8 = rng.gen_range(0, 3);
        if addins > 0 {
            buffer.push("mixed with".to_string());
            let mut adds: Vec<String> = Vec::new();
            for _ in 0..addins {
                adds.push(get_random_item(&materials.mixers));
            }
            buffer.push(adds.join(", "));
        }

        let topChance = rng.gen_range(1, 100);
        if topChance > 50 {
            buffer.push("topped with".to_string());
            buffer.push(get_random_item(&materials.toppers));
        }

        buffer.push("in a".to_string());
        buffer.push(get_random_item(&materials.glasses));


        return buffer.join(" ");

    }
    //end cocktail

    //5% chance
    return "is underage, and got NOTHIN'".to_string();
}
