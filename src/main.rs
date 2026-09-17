use std::{path::Path, rc::Rc};

use pack_create::{
    Card,
    CardRarityKind::{Basic, Common, Mythic, Rare, Uncommon},
};

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

    let mut basic = 0;
    let mut common = 0;
    let mut uncommon = 0;
    let mut rare = 0;
    let mut mythic = 0;

    for card in active_cube.ref_set.clone() {
        match card.rarirty_type {
            Basic => basic += 1,
            Common => common += 1,
            Uncommon => uncommon += 1,
            Rare => rare += 1,
            Mythic => mythic += 1,
        }
    }

    #[cfg(debug_assertions)]
    println!(
        "Found {} basics. Found {} commons. Found {} uncommons. Found {} rares. Found {} mythics.",
        basic, common, uncommon, rare, mythic
    );

    let pack = pack_create::Pack::new(Rc::new(active_cube));

    let mut clone_vec: Vec<usize> = Vec::new();
    for selected_card in pack.selected_cards.clone() {
        let card_number = selected_card.card.number;
        if clone_vec.contains(&card_number) {
            println!("Possible double card added, {}.", card_number);
        }
        clone_vec.push(card_number);
    }

    pack.selected_cards
        .iter()
        .enumerate()
        .for_each(|(index, card)| {
            println!("Slot {}: {}", index + 1, card);
        });
}
