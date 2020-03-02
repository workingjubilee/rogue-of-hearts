use bracket_lib::prelude::*;
use bracket_lib::prelude::BTerm;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket Lib!");
    }
}

fn main() {
    let context = BTerm::init_simple8x8(80, 50, "Hello Bracket Lib!", "resources");
    let gs = State{ };
    bracket_lib::prelude::main_loop(context, gs);
}

