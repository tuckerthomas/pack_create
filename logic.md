Overview:

14 Slots using the following categories:

#1-6 	6 	Common 	

#7 	1 	Common or The List[5]:
-    98.5% chance of a common from the set
-   1.5% chance of a Special Guest card

#8-10 	3 	Uncommon 	

#11 	1 	Non-foil Wildcard 	A card of any rarity from the set. Guaranteed to be non-foil.

#12 	1 	Rare or Mythic Rare:
-    87.5% chance of a rare
-   12.5% chance of a mythic rare

#13 	1 	Basic land 	In sets with no basic lands, this will be a common land.

#14 	1 	Foil Wildcard 	A card of any rarity from the set. Guaranteed to be foil.

#15 	1 	Token, art card, play aide 	Non-playable card, excluded for draft.:
-    65% – Token/Helper card
-   30% – Art card
-    5% – Art card with signature

Eventual Nice to Haves:
- Seeded packs
- CSV/Table input
- Web interface
- Fun name?

Input:
Set is a newline seperation of each instance of card: "[Quantity] [Card Name]\n"
- Since these cards are custom, we cannot use external data for creation, we also need to know their rarity

Output:
A setlist for the specific pack

Logic:

1. Collect the distribution and pool of cards from user

2. Validate the set

3. Begin card logic by creating a pack:
- For each category above:
    - Draw for the card from the rarity pool
    - Determine if a copy of the card has already been selected
    - Add it to the pack
- end after each category has been picked from

4. Print the pack

Assumptions:
- Some categories have specific logic, but we're excluding that for now.
