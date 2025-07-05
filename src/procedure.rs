///This module contains the procedures of the game, such as the player actions and book keeping functions that represent the actual game mechanics.

use crate::elements::*;
use crate::players::*;

// Player actions
pub fn player_move(player: &mut Player, new_position: usize) -> usize {
    player.set_position(new_position)
}
pub fn scavenge(player: &mut Player, ressource_pile: &mut Vec<Ressource>) -> bool {
    if ressource_pile.is_empty() {
        false
    } else {
        let ressource = ressource_pile.pop().unwrap();
        player.add_ressources(vec![ressource]);
        true
    }
}
pub fn share_ressources(player: &mut Player, other_player: &mut Player, ressource: RessourceType) -> bool {
    if let Some(index) = player.hand().iter().position(|r| r.ressource_type() == &ressource) {
        let ressource = player.hand().remove(index);
        other_player.add_ressources(vec![ressource]);
        true
    } else {
        false
    }
}
pub fn repair_room(player: &mut Player, rooms: &mut Vec<Room>, repair_field_index: usize) -> bool {
    let room = &mut rooms[player.position()];
    let repair_field = room.repair_field();
    if repair_field[repair_field_index].1 {
        player.remove_ressources(vec![Ressource::new(repair_field[repair_field_index].0.clone())]);
        true
    } else {
        return false
    }
}