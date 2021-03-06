use crate::{
    game::Game,
    components::Has,
};

#[derive(Clone, Copy)]
pub struct ControlBlock { }

impl ControlBlock {
    pub fn new() -> impl Fn(&mut Game, usize) -> () {
        move |game: &mut Game, entity: usize| -> () {
            game.world[entity] |= Has::ControlBlock as u32;
        }
    }
}
