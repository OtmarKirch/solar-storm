mod elements;
mod players;
mod game;

use elements::*;
use players::*;
use game::*;

fn main() {
    let rooms = dbg!(Room::init_rooms(Version::V1));
    //println!("First room, ability: {:?}", rooms[0]);
}
