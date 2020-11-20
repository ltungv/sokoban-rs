use ggez::{graphics, mint};

pub enum RenderableKind {
    Static,
    Animated,
}

/// Path to the image that represents the entity
pub struct Renderable<T> {
    drawables: Vec<T>,
}

impl<D> Renderable<D>
where
    D: graphics::Drawable + Clone,
{
    pub fn new_static(drawable: D) -> Self {
        Self {
            drawables: vec![drawable],
        }
    }

    pub fn new_animated(drawables: Vec<D>) -> Self {
        Self { drawables }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.drawables.len() {
            0 => panic!("Invalid RenderableKind"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn drawable(&self, idx: usize) -> D {
        self.drawables[idx % self.drawables.len()].clone()
    }
}

/// Position of the entity on the game's grid
pub type Position = mint::Point3<u8>;

pub type Scale = mint::Vector2<f32>;

#[derive(PartialEq)]
pub enum BoxColor {
    Blue,
    Red,
}

/// Marks an entity as a box
pub struct Box {
    pub color: BoxColor,
}

/// Marks an entity as a location where a box can be put into
pub struct BoxSpot {
    pub color: BoxColor,
}

/// Marks an entity as a player
#[derive(Default)]
pub struct Player;

/// Marks an entity as a wall
#[derive(Default)]
pub struct Wall;

#[derive(Default)]
pub struct Movable;

#[derive(Default)]
pub struct Immovable;
