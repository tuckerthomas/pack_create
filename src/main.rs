use std::{path::Path, rc::Rc};

use pack_create::Card;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    
    let path = Path::new(&args.path);

    println!("Opening CSV file from: '{:?}'.", path);

    let mut rdr = csv::Reader::from_path(path).expect("File not found");

    let mut cards: Vec<Card> = Vec::new();

    for result in rdr.deserialize() {
        let card: Card = result.expect("Could not deserialize");
        //println!("{:?}", card);
        cards.push(card);
    }

    let active_cube = pack_create::Cube::new(&cards);

    let pack = pack_create::Pack::new(Rc::new(active_cube));

    pack.selected_cards.iter().for_each(|card| {
        println!("{}", card);
    });
}
