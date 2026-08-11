use core::fmt;
use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use serde::Deserialize;

use rand::{RngExt, seq::IndexedRandom};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Card {
    #[serde(rename = "Card #")]
    number: usize,

    #[serde(rename = "Card Name")]
    name: String,

    #[serde(rename = "Card Rarity")]
    pub rarirty_type: CardRarityKind,

    #[serde(rename = "Card Type")]
    base: CardBaseKind,
}

impl Card {
    pub fn new(name: &str, number: usize, rarity_type: CardRarityKind, base: CardBaseKind) -> Self {
        Self {
            name: name.to_owned(),
            number: number,
            rarirty_type: rarity_type,
            base: base,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Card Number: {},\t\tName: {},\t\t\t\tRarity: {}, Base: {:?}]",
            self.number, self.name, self.rarirty_type, self.base
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Default)]
pub enum CardBaseKind {
    Land,
    #[serde(alias = "")]
    #[default]
    Unknown,
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
    pub fn new(variant: VariantKind, variant_chance: f64) -> Self {
        Self {
            variant,
            variant_chance,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SelectedCard {
    variants: Vec<VariantKind>,
    card: Rc<Card>,
}

impl SelectedCard {
    pub fn new(variants: Vec<VariantKind>, card: Rc<Card>) -> Self {
        Self {
            variants: variants,
            card: card,
        }
    }
}

impl fmt::Display for SelectedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ Special Tags {:?},\t\t\tCard Specifics {}]",
            self.variants, self.card
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Default)]
pub enum CardRarityKind {
    Basic,
    #[default]
    Common,
    Uncommon,
    Rare,
    Mythic,
}

impl fmt::Display for CardRarityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            match self {
                CardRarityKind::Basic => "Basic",
                CardRarityKind::Common => "Common",
                CardRarityKind::Uncommon => "Uncommon",
                CardRarityKind::Rare => "Rare",
                CardRarityKind::Mythic => "Mythic",
            }
        )
    }
}

#[derive(Debug)]
pub struct Cube {
    number_of_cards: usize,
    set: Vec<Card>,
    pub ref_set: Vec<Rc<Card>>,
    // A collection of RarityKinds that are included in the cube
    rarity_pool: RarityPool,
    // A collection of CardBaseKinds that are included in the cube
    base_pool: Vec<CardBaseKind>,
}

impl Cube {
    pub fn new(new_set: &Vec<Card>) -> Self {
        let temp_set = new_set.to_vec();

        let ref_set = temp_set.clone().into_iter().map(Rc::new).collect();

        let rarity_pool = RarityPool::new(&ref_set);

        let base_pool: Vec<CardBaseKind> = temp_set
            .clone()
            .into_iter()
            .map(|card| card.base.clone())
            .collect();

        Self {
            number_of_cards: temp_set.len(),
            set: temp_set,
            ref_set: ref_set,
            rarity_pool: rarity_pool,
            base_pool: base_pool,
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {:?}, {:?}, {:?}, {:?})",
            self.number_of_cards, self.set, self.ref_set, self.rarity_pool, self.base_pool
        )
    }
}

#[derive(Debug)]
struct RarityPool {
    basic: Vec<Rc<Card>>,
    common: Vec<Rc<Card>>,
    uncommon: Vec<Rc<Card>>,
    rare: Vec<Rc<Card>>,
    mythic: Vec<Rc<Card>>,
}

impl RarityPool {
    pub fn new(active_set: &Vec<Rc<Card>>) -> Self {
        let mut basic: Vec<Rc<Card>> = active_set.clone();
        basic.retain(|card_ref| card_ref.clone().rarirty_type == CardRarityKind::Basic);

        let mut common: Vec<Rc<Card>> = active_set.clone();
        common.retain(|card_ref| card_ref.clone().rarirty_type == CardRarityKind::Common);

        let mut uncommon: Vec<Rc<Card>> = active_set.clone();
        uncommon.retain(|card_ref| card_ref.clone().rarirty_type == CardRarityKind::Uncommon);

        let mut rare: Vec<Rc<Card>> = active_set.clone();
        rare.retain(|card_ref| card_ref.clone().rarirty_type == CardRarityKind::Rare);

        let mut mythic: Vec<Rc<Card>> = active_set.clone();
        mythic.retain(|card_ref| card_ref.clone().rarirty_type == CardRarityKind::Mythic);

        Self {
            basic: basic,
            common: common,
            uncommon: uncommon,
            rare: rare,
            mythic: mythic,
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

impl Pack {
    pub fn new(active_cube: Rc<Cube>) -> Self {
        let exlcuded_cards: RefCell<Vec<Rc<Card>>> = RefCell::new(Vec::new());

        let mut rng: rand::rngs::SmallRng = rand::make_rng();

        let fun_booster_variant_options = vec![
            VariantOption::new(VariantKind::Foil, 0.015),
            VariantOption::new(VariantKind::AltArt, 0.015),
        ];

        let static_foil_variant_options = vec![
            VariantOption::new(VariantKind::Foil, 1.0),
            VariantOption::new(VariantKind::AltArt, 0.015),
        ];

        let common_rarities = vec![CardRarityKind::Common];
        let uncommon_rarities = vec![CardRarityKind::Uncommon];

        // Common
        let slot_1: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            None,
            common_rarities.clone(),
            None,
        ));
        let slot_2: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities.clone(),
            None,
        ));
        let slot_3: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities.clone(),
            None,
        ));
        let slot_4: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities.clone(),
            None,
        ));
        let slot_5: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities.clone(),
            None,
        ));
        let slot_6: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities.clone(),
            None,
        ));
        let slot_7: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            common_rarities,
            None,
        ));

        // Uncommon
        let slot_8: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            uncommon_rarities.clone(),
            None,
        ));
        let slot_9: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            uncommon_rarities.clone(),
            None,
        ));
        let slot_10: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            uncommon_rarities,
            None,
        ));

        // Any card
        let slot_11: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options.clone(),
            exlcuded_cards.clone(),
            None,
            None,
        ));

        // Mythic or Rare: 87.5% Rare, 12.5% Mythic Rare
        let slot_12: Rc<SelectedCard> = if rng.random_bool(0.125) {
            Rc::new(draw_card_from_active_set(
                &active_cube,
                fun_booster_variant_options.clone(),
                exlcuded_cards.clone(),
                vec![CardRarityKind::Mythic],
                None,
            ))
        } else {
            Rc::new(draw_card_from_active_set(
                &active_cube,
                fun_booster_variant_options.clone(),
                exlcuded_cards.clone(),
                vec![CardRarityKind::Rare],
                None,
            ))
        };

        // Any Land card, 50% chance to be Common, 45% chance to be basic, %5 chance to be rare
        let mut test_rng: rand::rngs::SmallRng = rand::make_rng();
        let slot_13_chance: u32 = test_rng.random_range(1..=100);
        let slot_13_rarity = match slot_13_chance {
            1..=50 => CardRarityKind::Common,
            51..=94 => CardRarityKind::Basic,
            95..=100 => CardRarityKind::Rare,
            _ => CardRarityKind::Common,
        };

        #[cfg(debug_assertions)]
        println!(
            "Using {} chance and {} rarity for slot 13",
            slot_13_chance, slot_13_rarity
        );

        let slot_13: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            fun_booster_variant_options,
            exlcuded_cards.clone(),
            vec![slot_13_rarity],
            vec![CardBaseKind::Land],
        ));

        // Any card
        let slot_14: Rc<SelectedCard> = Rc::new(draw_card_from_active_set(
            &active_cube,
            static_foil_variant_options,
            exlcuded_cards.clone(),
            None,
            None,
        ));

        let card_pack: [Rc<SelectedCard>; 14] = [
            slot_1, slot_2, slot_3, slot_4, slot_5, slot_6, slot_7, slot_8, slot_9, slot_10,
            slot_11, slot_12, slot_13, slot_14,
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

pub fn draw_card_from_active_set(
    active_cube: &Cube,
    optional_variant_options: impl Into<Option<Vec<VariantOption>>>,
    optional_excluded_cards: impl Into<Option<RefCell<Vec<Rc<Card>>>>>,
    optional_card_rarities: impl Into<Option<Vec<CardRarityKind>>>,
    optional_bases: impl Into<Option<Vec<CardBaseKind>>>,
) -> SelectedCard {
    let rarities: Option<Vec<CardRarityKind>> = optional_card_rarities.into();
    let excluded_cards: Option<RefCell<Vec<Rc<Card>>>> = optional_excluded_cards.into();
    let variant_options: Option<Vec<VariantOption>> = optional_variant_options.into();
    let base_cards: Option<Vec<CardBaseKind>> = optional_bases.into();

    let mut pool_set: Vec<Rc<Card>> = active_cube.ref_set.clone(); // This clone seems costly >.>;

    let mut variants: Vec<VariantKind> = Vec::new();

    // If there are variants specifiied, fill the draw pool with those specific varients given their probability
    if let Some(variant_options) = variant_options {
        let mut rng1: rand::rngs::SmallRng = rand::make_rng();
        variants = variant_options
            .iter()
            .filter_map(|vo| {
                if rng1.random_bool(vo.variant_chance) {
                    Some(vo.variant)
                } else {
                    None
                }
            })
            .collect();
    }

    // If excluded cards exists, they must be removed from the pool.
    if let Some(mut excluded_cards) = excluded_cards.clone() {
        // Convert to a hashset for quick lookup
        let remove_set: HashSet<_> = excluded_cards.get_mut().iter().collect();
        // Keep the card if it doesnt exist in the list of cards to remove
        pool_set.retain(|card| !remove_set.contains(card));
    }

    // If rarity is defined, keep the cards that match
    if let Some(rarities) = rarities {
        let mut total_temp_set = Vec::new();
        let mut temp_set = Vec::new();
        for rarity in rarities {
            temp_set = match rarity {
                CardRarityKind::Basic => active_cube.rarity_pool.basic.clone(),
                CardRarityKind::Common => active_cube.rarity_pool.common.clone(),
                CardRarityKind::Uncommon => active_cube.rarity_pool.uncommon.clone(),
                CardRarityKind::Rare => active_cube.rarity_pool.rare.clone(),
                CardRarityKind::Mythic => active_cube.rarity_pool.mythic.clone(),
            };

            total_temp_set.append(&mut temp_set);
        }

        pool_set.retain(|card| total_temp_set.contains(card));
    }

    // If a base is given, remove cards that are not that base
    if let Some(base_cards) = base_cards.clone() {
        // Keep the card if it doesnt exist in the list of cards to remove
        pool_set.retain(|card| base_cards.contains(&card.base));
    }

    let mut rng: rand::rngs::SmallRng = rand::make_rng();

    let chosen_card = pool_set
        .choose(&mut rng)
        .expect("Unable to choose card via draw.");

    if let Some(mut excluded_cards) = excluded_cards {
        excluded_cards.get_mut().push(chosen_card.clone());
    }

    SelectedCard::new(variants, chosen_card.clone())
}
