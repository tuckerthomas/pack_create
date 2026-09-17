# pack_create

Rust CLI that generates a random 14-card booster pack from a cube defined in a CSV file.

## Pack layout (current implementation — 14 slots)

| Slot | Draw | Notes |
| ---- | ---- | ----- |
| 1–6  | Common | |
| 7    | Common | Spec'd 1.5% Special Guest roll not implemented |
| 8–10 | Uncommon | |
| 11   | Any card | Uses fun-booster variants (1.5% foil chance); spec's "guaranteed non-foil" not honored |
| 12   | 87.5% Rare / 12.5% Mythic | |
| 13   | Land; 50% Common / 44% Basic / 6% Rare | Roll ranges: 1–50 → Common, 51–94 → Basic, 95–100 → Rare. Panics if no lands of the rolled rarity exist |
| 14   | Any card, foil guaranteed | Plus independent 1.5% alt-art roll |

A 15th category (token/art card/play aide, 65%/30%/5%) exists in the original spec but is not implemented; the pack array is fixed at 14.

## Variants

Each drawn card independently rolls each variant option for its slot:

- **Fun-booster slots** (1–11, 13): 1.5% foil, 1.5% alt-art — a card can get both
- **Slot 14**: 100% foil, 1.5% alt-art

## Duplicate prevention

Every drawn card is appended to a shared exclusion list; subsequent draws filter it out using card identity (number, name, rarity, and type all equal). No two slots can hold the same card. (A legacy debug print in `main.rs` warns on duplicate card numbers but is unreachable.)

## Input

CSV file path, required, via `--path` / `-p`. Columns:

| Column | Values |
| ------ | ------ |
| `Card #` | Integer |
| `Card Name` | String |
| `Card Rarity` | `Basic` \| `Common` \| `Uncommon` \| `Rare` \| `Mythic` |
| `Card Type` | `Land` or empty (→ `Unknown`) |

Panics (no row detail) on: missing file, malformed row, and empty draw pools (e.g., a cube with no uncommons panics on slots 8–10; a cube with no rares panics on slot 12). There is no up-front set validation.

## Output

One line per slot: `Slot {n}: {variant tags} {card details}`.

Debug builds additionally print per-rarity counts and the slot 13 roll.

## Pipeline

1. Parse CSV → cards
2. Build cube: card list + per-rarity pools
3. Draw 14 slots in order (exclusion list prevents duplicates)
4. Print the pack

## Not implemented / future

- **Slot 7**: 1.5% Special Guest card (definition needed)
- **Slot 13**: basic land with fallback to common land when no basics exist
- **Slot 15**: token/art card/play aide (65% token, 30% art, 5% signed art)
- Seeded packs
- CSV/table input improvements (better error reporting)
- Web interface
