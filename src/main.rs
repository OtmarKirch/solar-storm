mod elements;
mod players;
mod game;
mod procedure;

use elements::*;
use players::*;
use game::*;
use procedure::*;

fn main() {
    let rooms = dbg!(Room::init_rooms(Version::V1));
    //println!("First room, ability: {:?}", rooms[0]);
}
