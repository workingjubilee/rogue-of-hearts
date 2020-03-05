

// Wow I have cut a huge pile of work out for myself huh.

// A primary input to accuracy.
// player should be able to issue commands to allies, this affects responsiveness.
// I might as well generate formal Actors with mailboxes and everything huh.

pub struct Actor {
    erg: i32, // Energy
    mood: Mood,
    commandable: bool,
    last_command: Command,
    next_reval: i32
}

struct Command {
    intent: Intent,
    target: i32, // let's pretend we are targeting an object ID
}

enum Mood {
    Big
}

// Describes the intents that can be given by commands.
// Does not have to describe all intents a character can have,
// only the ones that can be requested by other entities
enum Intent {
    Attack,
    Retreat,
    Defend
}

// movement NSWE costs 10e
// movement NE / SE / SW / SE costs 14e
// everyone with a positive energy balance acts
// attacking costs ???
// going negative means you wait until you have enough energy again
// people gain 1e on every tick

enum Action {
    Ready,
    NeedEnergy
}

// how can I efficiently allocate energy to multiple entities?
// meh, not worth it, just advance tick by tick until Action::Ready
// else is premature optimization
// more likely to be useful: iterate data but don't rerender until Action::Ready
// or other condition is met.