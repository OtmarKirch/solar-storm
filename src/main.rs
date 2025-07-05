mod elements;
mod players;
mod game;
mod mechanics;

use elements::*;
use players::*;
use game::*;
use mechanics::*;

fn main() {
    let rooms = dbg!(Room::init_rooms(Version::V1));
    //println!("First room, ability: {:?}", rooms[0]);
}
