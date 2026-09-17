# pack_create

A Rust CLI tool that generates random 14-card booster packs from a user-defined Magic: The Gathering cube.

## Features

- Reads a cube from a CSV file with card name, number, rarity, and type
- Generates packs following a configurable draft format (14 slots)
- Duplicate prevention across all slots in a pack
- Variant rolls (foil and alt-art) with configurable probabilities
- Rarity-weighted draws for land and mythic slots

## Building

```bash
cargo build --release
```

The release profile is optimized for small binary size (`opt-level = "z"` with LTO).

## Usage

```bash
./target/release/pack_create --path path/to/cube.csv
```

or

```bash
./target/release/pack_create -p path/to/cube.csv
```

## CSV Input Format

The input file must contain the following columns:

| Column | Description | Example |
|--------|-------------|---------|
| `Card #` | Unique integer identifier for the card | `1001` |
| `Card Name` | Name of the card | `Lightning Bolt` |
| `Card Rarity` | One of: `Basic`, `Common`, `Uncommon`, `Rare`, `Mythic` | `Rare` |
| `Card Type` | `Land` or empty for non-land cards | `Land` |

## Pack Layout (14 Slots)

| Slot | Draw | Probability |
|------|------|-------------|
| 1–7 | Common | — |
| 8–10 | Uncommon | — |
| 11 | Any card (wildcard) | — |
| 12 | Rare or Mythic | 87.5% Rare / 12.5% Mythic |
| 13 | Land | 50% Common / 44% Basic / 6% Rare |
| 14 | Any card (foil guaranteed) | — |

### Variant Rolls

- **Slots 1–13**: Independent 1.5% foil and 1.5% alt-art rolls per card
- **Slot 14**: 100% foil, plus an independent 1.5% alt-art roll

## Output

One line per slot showing the variant tags and card details:

```
Slot 1: [ ] [Card Number: 42, Name: Stone Wall, Rarity: (Common), Base: Unknown]
Slot 2: [Foil ] [Card Number: 105, Name: Elven Archer, Rarity: (Common), Base: Unknown]
...
Slot 14: [Foil Alt-Art ] [Card Number: 203, Name: Fireball, Rarity: (Rare), Base: Unknown]
```

In debug builds, additional diagnostic output is printed including per-rarity card counts and the slot 13 land roll.

## Technical Details

- **Rust edition**: 2024
- **Dependencies**: `clap` (CLI parsing), `csv` (input parsing), `rand` (random draws), `serde` (deserialization)
- Cards are stored using `Rc<Card>` for efficient sharing across draw operations
- An exclusion list tracks drawn cards to prevent duplicates within a single pack

## Roadmap

- Implement the 1.5% "Special Guest" card roll for slot 7
- Add fallback for slot 13 when no basic lands exist in the cube
- Implement slot 15 (token/art card/play aide)
- Seeded pack generation for reproducibility
- Improved error reporting with specific row/card details
- Web interface

## License

See `Cargo.toml` for package metadata.
