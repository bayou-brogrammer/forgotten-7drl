# forgotten-7drl

[![dependency status](https://deps.rs/repo/github/lecoqjacob/forgotten-7drl/status.svg)](https://deps.rs/repo/github/lecoqjacob/forgotten-7drl)

a roguelike-game written in Rust for the monthly 7DRL game jam.

# Story
You try one final time to page command center, but all you hear is static. 
You stare at the last MRE and water bottle in the storage locker. It has been 3 weeks since the last communication with home base.

You are the last survivor of the team tasked to destroy the core reactor powering the robots. It has
been 3 long years at war. They have been relentless in their pursuit of the last human resistance.
This is the last resort. If you fail, so does humanity

You page one more time.....nothing.

You are forgotten.

# Combat
Each weapon has a DMG(♥) and PEN(♦) stat, and each enemy has heatlh(♥) and armour(♦)

If an enemy is hit with a weapon that has a higher PEN than their armour, their health is
reduced by the weapon's DMG. If a projectile's PEN exceeds an enemy's armour, it continues
on its path with its PEN reduced by the enemy's armour.

# Enemies

| Enemy | Description                                               |   
|-------|-----------------------------------------------------------|
|Minibot|basic guard robot                                          |
|SecBot |upgraded minibot                                           |
|RoboCop|elite guard robot. Alerts nearby robots when it sees you   |
|DoomCop|Kill bot. Very hard to kill. Explodes on death             |

# Controls
## Keyboard

| Action                    | Key               |   
|---------------------------|-------------------|
|Movement/Aim               |Arrows/WASD/HJKL   |
|Cancel Aim                 |Escape             |
|Wait                       |Space              |
|Examine                    |X                  |
|Descend                    |Period             |
|Get Weapon                 |G                  |
|Fire Ranged Weapon Slot 1  |1                  |
|Fire Ranged Weapon Slot 2  |2                  |
|Fire Ranged Weapon Slot 3  |3                  |

## Gamepad

| Action                    | Key       |   
|---------------------------|-----------|
|Movement/Aim               |D-Pad      |
|Cancel Aim                 |Select     |
|Wait                       |Select     |
|Examine                    |Select     |
|Descend                    |Select     |
|Get Weapon                 |Select     |
|Fire Ranged Weapon Slot 1  |X/Square   |
|Fire Ranged Weapon Slot 2  |A/Cross    |
|Fire Ranged Weapon Slot 3  |B/Circle   |
