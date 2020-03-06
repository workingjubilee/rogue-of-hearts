extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Monster, Name, Point, console};

pub struct DecisionMaker {}

impl<'a> System<'a> for DecisionMaker {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>, 
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data : Self::SystemData) {

        let (player_pos, viewshed, monster, name) = data;

        for (viewshed,_monster,name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}

// Wow I have cut a huge pile of work out for myself huh.

// A primary input to accuracy.
// acc_mod: float multiplied by acc_base: float to get acc?
// acc_mod altered by "nerves"/"morale"

// player should be able to issue commands to allies
// responsiveness is mostly-deterministic but mediated by behavior loop.
// "might disobey" and "disobey %" stored separately.

// I might as well generate formal Actors with mailboxes and everything huh.


// pub struct Actor {
//     erg: i32, // Energy
//     mood: Mood,
//     commandable: bool,
//     last_command: Command,
//     next_reval: i32,
// }

// struct Command {
//     intent: Intent,
//     target: i32, // let's pretend we are targeting an object ID
// }

// enum Mood {
//     Big,
// }

// // Describes the intents that can be given by commands.
// // Does not have to describe all intents a character can have,
// // only the ones that can be requested by other entities
// enum Intent {
//     Attack,
//     Retreat,
//     Defend,
//     None,
// }

// // movement NSWE costs 10e
// // movement NE / SE / SW / SE costs 14e
// // everyone with a positive energy balance acts
// // attacking costs ???
// // going negative means you wait until you have enough energy again
// // people gain 1e on every tick

// enum Action {
//     Ready,
//     NeedEnergy,
// }

// // how can I efficiently allocate energy to multiple entities?
// // meh, not worth it, just advance tick by tick until Action::Ready
// // else is premature optimization
// // more likely to be useful: iterate data but don't rerender until Action::Ready
// // or other condition is met.
