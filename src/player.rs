use bracket_lib::prelude::{VirtualKeyCode, BTerm};
extern crate specs;
use super::{Map, Player, Position, State, TileType, Viewshed};
use specs::prelude::*;
use std::cmp::{max, min};

enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SW,
    SE,
}


fn try_move_player(dir: Direction, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let map = ecs.fetch::<Map>();
    let (delta_x, delta_y) = match dir {
        Direction::E => (1, 0),
        Direction::S => (0, 1),
        Direction::W => (-1, 0),
        Direction::N => (0, -1),
        Direction::SE => (1, 1),
        Direction::SW => (-1, 1),
        Direction::NE => (1, -1),
        Direction::NW => (-1, -1),
    };

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {

            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(Direction::W, &mut gs.ecs),
            VirtualKeyCode::Numpad4 => try_move_player(Direction::W, &mut gs.ecs),
            VirtualKeyCode::H => try_move_player(Direction::W, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(Direction::E, &mut gs.ecs),
            VirtualKeyCode::Numpad6 => try_move_player(Direction::E, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(Direction::E, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(Direction::N, &mut gs.ecs),
            VirtualKeyCode::Numpad8 => try_move_player(Direction::N, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player(Direction::N, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(Direction::S, &mut gs.ecs),
            VirtualKeyCode::Numpad2 => try_move_player(Direction::S, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(Direction::S, &mut gs.ecs),
            _ => {}
        },
    }
}
