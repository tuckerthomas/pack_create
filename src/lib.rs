use core::fmt;
use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use serde::{Deserialize};

use rand::{RngExt, seq::IndexedRandom};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Card {
    #[serde(rename = "Card #")]
    number: usize,
    #[serde(rename = "Card Name")]
    name: String,
    #[serde(rename = "Card Rarity")]
    rarirty_type: CardRarirtyKind,
}

impl Card {
    pub fn new (name: &str, number: usize, rarity_type: CardRarirtyKind) -> Self {
        Self { name: name.to_owned(), number: number, rarirty_type: rarity_type }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Card Number: {},\t\tName: {},\t\t\t\tRarity: {}]", self.number, self.name, self.rarirty_type)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum VariantKind {
    Foil,
    AltArt,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VariantOption {
    variant: VariantKind,
    variant_chance: f64,
}

impl VariantOption {
    pub fn new(variant: VariantKind, variant_chance: f64) -> Self{
        Self {
            variant,
            variant_chance
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SelectedCard {
    variants: Vec<VariantKind>,
    card: Rc<Card>
}

impl SelectedCard {
    pub fn new(variants: Vec<VariantKind>, card: Rc<Card>) -> Self {
        Self {
            variants: variants,
            card: card
        }
    }
}

impl fmt::Display for SelectedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ Special Tags {:?},\t\t\tCard Specifics {}]", self.variants, self.card)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Default)]
pub enum CardRarirtyKind {
    Basic,
    #[default]
    Common,
    Uncommon,
    Rare,
    Mythic,
}

impl fmt::Display for CardRarirtyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", match self {
            CardRarirtyKind::Basic => "Basic",
            CardRarirtyKind::Common => "Common",
            CardRarirtyKind::Uncommon => "Uncommon",
            CardRarirtyKind::Rare => "Rare",
            CardRarirtyKind::Mythic => "Mythic",
        })
    }
}

#[derive(Debug)]
pub struct Cube {
    number_of_cards: usize,
    set: Vec<Card>,
    ref_set: Vec<Rc<Card>>,
    rarity_pool: RarityPool
}

impl Cube {
    pub fn new(new_set: &Vec<Card>) -> Self {
        let temp_set = new_set.to_vec();

        let ref_set = temp_set.clone().into_iter().map(Rc::new).collect();

        let rarity_pool = RarityPool::new(&ref_set);

        Self {
            number_of_cards: temp_set.len(),
            set: temp_set,
            ref_set: ref_set,
            rarity_pool: rarity_pool,
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:?}, {:?}, {:?})", self.number_of_cards, self.set, self.ref_set, self.rarity_pool)
    }
}

#[derive(Debug)]
struct RarityPool {
    basic: Vec<Rc<Card>>,
    common: Vec<Rc<Card>>,
    uncommon: Vec<Rc<Card>>,
    rare: Vec<Rc<Card>>,
    mythic: Vec<Rc<Card>>
}

impl RarityPool {
    pub fn new(active_set: &Vec<Rc<Card>>) -> Self {
        let mut basic: Vec<Rc<Card>> = active_set.clone();
        basic.retain(|card_ref| card_ref.clone().rarirty_type == CardRarirtyKind::Basic);

        let mut common: Vec<Rc<Card>> = active_set.clone();
        common.retain(|card_ref| card_ref.clone().rarirty_type == CardRarirtyKind::Common);

        let mut uncommon: Vec<Rc<Card>> = active_set.clone();
        uncommon.retain(|card_ref| card_ref.clone().rarirty_type == CardRarirtyKind::Uncommon);

        let mut rare: Vec<Rc<Card>> = active_set.clone();
        rare.retain(|card_ref| card_ref.clone().rarirty_type == CardRarirtyKind::Rare);

        let mut mythic: Vec<Rc<Card>> = active_set.clone();
        mythic.retain(|card_ref| card_ref.clone().rarirty_type == CardRarirtyKind::Mythic);

        Self {
            basic: basic,
            common: common,
            uncommon: uncommon,
            rare: rare,
            mythic: mythic
        }
    }
}

/*

14 Slots

1-6 Common

7 Common or The List

8 Uncommon

11 Non-foil wildcard - Any card from the set

12 Rare or Mythic rare, 87.5% Rare, 12.5% Mythic Rare

13 Basic Land

14 Foil wildcard - Any card from the set

*/
#[derive(Debug)]
pub struct Pack {
    pub selected_cards: [Rc<SelectedCard>; 14],
    cube: Rc<Cube>,
}

impl Pack{
    pub fn new(active_cube: Rc<Cube>) -> Self {
        let exlcuded_cards: RefCell<Vec<Rc<Card>>> = RefCell::new(Vec::new());

        let mut rng: rand::rngs::SmallRng = rand::make_rng();

        let fun_booster_variant_options= vec![
            VariantOption::new(VariantKind::Foil, 0.015), 
            VariantOption::new(VariantKind::AltArt, 0.015)
        ];

        let static_foil_variant_options= vec![
            VariantOption::new(VariantKind::Foil, 1.0),
            VariantOption::new(VariantKind::AltArt, 0.015)
        ];

        // Common
        let slot_1: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), None, CardRarirtyKind::Common));
        let slot_2: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        let slot_3: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        let slot_4: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        let slot_5: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        let slot_6: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        let slot_7: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Common));
        
        // Uncommon
        let slot_8: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Uncommon));
        let slot_9: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Uncommon));
        let slot_10: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Uncommon));
        
        // Any card
        let slot_11: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube,fun_booster_variant_options.clone(), exlcuded_cards.clone(), None));

        // Mythic or Rare: 87.5% Rare, 12.5% Mythic Rare
        let slot_12: Rc<SelectedCard> = if rng.random_bool(0.125) {
            Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Mythic))
        } else {
            Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options.clone(), exlcuded_cards.clone(), CardRarirtyKind::Rare))
        };

        // Basic Land or Common Land
        let slot_13: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube, fun_booster_variant_options, exlcuded_cards.clone(), CardRarirtyKind::Basic));

        // Any card
        let slot_14: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(&active_cube,static_foil_variant_options, exlcuded_cards.clone(), None));
        

        let card_pack: [Rc<SelectedCard>; 14] = [
            slot_1,
            slot_2,
            slot_3,
            slot_4,
            slot_5,
            slot_6,
            slot_7,
            slot_8,
            slot_9,
            slot_10,
            slot_11,
            slot_12,
            slot_13,
            slot_14
        ];
        let new_pack = Self {
            selected_cards: card_pack.clone(),
            cube: active_cube.clone(),
        };
        return new_pack;
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, excluding cube)", self.selected_cards)
    }
}

pub fn draw_card_from_active_set(active_cube: &Cube, optional_variant_options: impl Into<Option<Vec<VariantOption>>>, optional_excluded_cards: impl Into<Option<RefCell<Vec<Rc<Card>>>>>, optional_card_rarity: impl Into<Option<CardRarirtyKind>>) -> SelectedCard {
    let rarity: Option<CardRarirtyKind> = optional_card_rarity.into();
    let excluded_cards: Option<RefCell<Vec<Rc<Card>>>> = optional_excluded_cards.into();
    let variant_options: Option<Vec<VariantOption>> = optional_variant_options.into();

    let mut pool_set: Vec<Rc<Card>> = active_cube.ref_set.clone(); // This clone seems costly >.>;

    let mut variants: Vec<VariantKind> = Vec::new();

    if let Some(variant_options) = variant_options {
        let mut rng1: rand::rngs::SmallRng = rand::make_rng();
        variants = variant_options.iter().filter_map(|vo| {
            if rng1.random_bool(vo.variant_chance) {
                Some(vo.variant)
            } else {
                None
            }
        }).collect();
    }
    
    // If rarity is defined, remove the cards that match 
    if let Some(rarity) = rarity {
        pool_set = match rarity {
            CardRarirtyKind::Basic => active_cube.rarity_pool.basic.clone(),
            CardRarirtyKind::Common => active_cube.rarity_pool.common.clone(),
            CardRarirtyKind::Uncommon => active_cube.rarity_pool.uncommon.clone(),
            CardRarirtyKind::Rare => active_cube.rarity_pool.rare.clone(),
            CardRarirtyKind::Mythic => active_cube.rarity_pool.mythic.clone(),
            _ => active_cube.ref_set.clone(),
        };
    }
    
    // If excluded cards exists, they must be removed from the pool.
    if let Some(mut excluded_cards) = excluded_cards.clone() {
        // Convert to a hashset for quick lookup
        let remove_set: HashSet<_> = excluded_cards.get_mut().iter().collect();
        // Keep the card if it doesnt exist in the list of cards to remove
        pool_set.retain(|card| !remove_set.contains(card));
    }

    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    
    let chosen_card = pool_set.choose(&mut rng).expect("Unable to choose card via draw.");
    
    if let Some(mut excluded_cards) = excluded_cards {
        excluded_cards.get_mut().push(chosen_card.clone());
    }
    
    SelectedCard::new(variants, chosen_card.clone())
}