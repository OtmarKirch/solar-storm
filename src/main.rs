mod elements
;

use elements::*;

fn main() {
    let rooms = dbg!(Room::init_rooms(Version::V1));
    //println!("First room, ability: {:?}", rooms[0]);
}
