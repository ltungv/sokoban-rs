use ggez::{graphics, mint};

/// Determines whether an entity can be animated
pub enum RenderableKind {
    Static,
    Animated,
}

/// Contains a list of objects that can be drawn to screen to represent an entity
pub struct Renderable<T> {
    drawables: Vec<T>,
}

impl<D> Renderable<D>
where
    D: graphics::Drawable + Clone,
{
    /// Create a new `Drawable` components that can not be animated
    pub fn new_static(drawable: D) -> Self {
        Self {
            drawables: vec![drawable],
        }
    }

    /// Create a new `Drawable` components that can be animated using the list of sprites
    pub fn new_animated(drawables: Vec<D>) -> Self {
        Self { drawables }
    }

    /// Return whether the `Renderable` is static or animated
    pub fn kind(&self) -> RenderableKind {
        match self.drawables.len() {
            0 => panic!("Invalid RenderableKind"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    /// Return the `Drawable` object at the given index
    pub fn drawable(&self, idx: usize) -> D {
        self.drawables[idx % self.drawables.len()].clone()
    }
}

/// Determines the position of an entity on the game map
pub type Position = mint::Point3<u8>;

/// Determines the scaling of an entity when being rendered to screeen
pub type Scale = mint::Vector2<f32>;

/// Determines the color of a `Box` or `BoxSpot`
#[derive(PartialEq)]
pub enum BoxColor {
    Blue,
    Red,
}

/// Marks an entity to be a box
pub struct Box {
    pub color: BoxColor,
}

/// Marks an entity to be a location where a box can be put into
pub struct BoxSpot {
    pub color: BoxColor,
}

/// Marks an entity to be a player
#[derive(Default)]
pub struct Player;

/// Marks an entity to be a wall
#[derive(Default)]
pub struct Wall;

/// Marks an entity to be movable
#[derive(Default)]
pub struct Movable;

/// Marks an entity to be immovable
#[derive(Default)]
pub struct Immovable;
