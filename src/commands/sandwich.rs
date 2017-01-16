#![feature(proc_macro)]

extern crate serde_json;
extern crate rand;
extern crate serenity;
use serenity::model::{Message};

use std::fs::File;
use self::rand::thread_rng;
use self::rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct Materials {
    meat: Vec<Item>,
    cheese: Vec<Item>,
    sauce: Vec<Item>,
    bread: Vec<Item>,
    cooking: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    cost: f64,
    rarity: f64
}

pub fn generate_sandwich(message: &serenity::model::Message) {

    let file = File::open("data/sandwiches.json").unwrap();
    let materials: Materials = serde_json::from_reader(file).unwrap();

    let mut rng = thread_rng();
    let mut cost: f64 = 0.0;
    let mut buffer: Vec<String> = Vec::new();
    buffer.push(format!("\n```\n  _ Bartender's Bistro _  \nOrder For: {}", message.author.name));

    for _ in 0..rng.gen_range(1, 3) {
        let process = get_material(&materials.meat, &cost);
        cost = cost + process.1;
        buffer.push(process.0);
    }
    for _ in 0..rng.gen_range(1, 2) {
        let process = get_material(&materials.cheese, &cost);
        cost = cost + process.1;
        buffer.push(process.0);
    }
    for _ in 0..rng.gen_range(1, 2) {
        let process = get_material(&materials.sauce, &cost);
        cost = cost + process.1;
        buffer.push(process.0);
    }

    let bread = get_extra(&materials.bread);
    let cooking = get_extra(&materials.cooking);

    buffer.push(format!("on {}, {}", bread, cooking));

    let total_padding = get_padding_length("Total:");
    buffer.push(format!("Total:{}${:.2}```", total_padding, cost));

    message.reply(buffer.join("\n").as_str());
}

fn get_padding_length(text: &str) -> String {
    let mut s = String::with_capacity(20);
    for _ in 0..(20-text.len()) {
        s.push(' ');
    }
    return s;
}

fn get_extra(items: &Vec<Item>) -> String {
    let index = thread_rng().gen_range(0, items.len()-1);
    let item = items.get(index).unwrap();
    return item.name.clone();
}

fn get_material(items: &Vec<Item>, cost: &f64) -> (String, f64) {
    let index = thread_rng().gen_range(0, items.len()-1);
    let item = items.get(index).unwrap();
    let ref name = item.name;
    let mut local_cost = item.cost;
    let padding: String = get_padding_length(&name);
    let x: String = format!("{}{}${:.2}", name, padding, &local_cost);
    return (x, local_cost);
}
