use bracket_lib::prelude::{BTerm, VirtualKeyCode};
extern crate specs;
use super::{Map, Player, Position, State, TileType, Viewshed, RunState, Point};
use specs::prelude::*;
use std::cmp::{max, min};

// Cute enum but might slow down system?
// enum Direction {
//     N,
//     S,
//     W,
//     E,
//     NE,
//     NW,
//     SW,
//     SE,
// }

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();
    
    // let (delta_x, delta_y) = match dir {
    //     Direction::E => (1, 0),
    //     Direction::S => (0, 1),
    //     Direction::W => (-1, 0),
        // Direction::N => (0, -1),
        // Direction::SE => (1, 1),
        // Direction::SW => (-1, 1),
        // Direction::NE => (1, -1),
        // Direction::NW => (-1, -1),
    // };

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Numpad4 | VirtualKeyCode::Left | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad6 | VirtualKeyCode::Right | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
    RunState::Running
}
