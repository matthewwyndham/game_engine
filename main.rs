// custom extensible game engine
// built so I can easily iterate games and add new features
// and perhaps add custom mods easily

/*
Features:
- TOP DOWN RPG (later add 1st person and 3rd person camera)
- Generate world based on seed & pack-list (reproducibly)
- DEFAULT BASE PACK:
    - Long form RPG, no roguelike features
    - Ability scores -> category of skills
    - Many skills
    - Perk points each level
    - Character Looks Customization (sims assets?)
    - Gameplay Systems:
        -- Melee (+shield, fast paced and up close, single/multi target)
        -- Ranged (bow&arrow, sling&stone, throwing weapons, slow, single target)
        -- Magic (spell system, direct magic use, complex, single/multi target)
        -- Runecrafting (craft one time use spell bombs, enchant weapons/armor)
        -- Forging (make weapons & armor)
        -- Stealth (pick pocket, avoid detection, dodge attacks) [Rogue Class]
        -- Charimsa (better prices, non-combat solutions, buff allies) [Musician Class]
        -- Fortitude (more health, take less damage from ranged, resist debuffs) [Berserker Class]
        -- Intellect (more MP, higher spell power/damage, more crit success) [Mage Class]
        -- Potency (more damage, more effect of use items, less health) [Silicomancer Class]

- ROGUELIKE BASE PACK:
    - Similar to default, but each character is a new run in a smaller world with a clear final boss
    - Pillars of a roguelike: Pickups, Combat, Procedural, Meta Progression, Permadeath,
    - PICKUPS:
        -| Consumable Permanent stat upgrades
        -| Replaceable Accessories (bag of buffs, adds new mechanics)
        -| Interchangeable Weapons (limited capacity, changes gameplay greatly)
        -| Character Basic Kits (some exclusive accessories/weapons/mechanics)
    - COMBAT:
        -| FPS/3rdPS? (consumable ammo)
        -| Melee (close range)
        -| Spell (no mana points? learn from spell scrolls instead of using them)
        -| Customizable skill hotkeys
        -| [[Enemies]]
    - PROCEDURAL:
        -| 7 Stage Types
        -| Final Boss
        -| Infinite Run
        -| Interactables
        -| Merchant
    - Meta Progression:
        -| Achievement System (mod-extensible)
        -| All pickups can be unlocked by default, or restricted to a condition
        -| Character Alternate Starting skills/items
        -| Spend meta-points to start a run strong
        -| Creative Mode (to test strategies, unlocked by beating boss 7 times?)
        -| (the only save file)
    - Permadeath (no saving characters, but achievements are saved right away, and all earned currency is added right away)
    - When you boot up the game there are no difficulty options or anything, you just jump in. 
        - Beat the boss on normal mode, unlock hard mode: faster & stronger enemies, more money, better loot, faster gameplay. 
        - Multiple difficulty levels. 
        - Final unlock is creative mode where you can experiment with different item combos.
*/

// roguelike game first
fn main() {

    // init game

    // show menu

    // load pack-list settings

    // START GAME

    // load base pack

    // load addon packs

    // game loop
}