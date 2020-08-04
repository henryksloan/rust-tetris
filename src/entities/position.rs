use amethyst::ecs::{Component, VecStorage};

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}
