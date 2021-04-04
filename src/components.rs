use ggez::mint;

/// This component determines if a renderable entity is rendered with a single resource (static)
/// it is rendered with multiple sources (animated).
pub enum RenderableKind {
    Static,
    Animated,
}

/// A renderable entity can be drawn on to the game screen.
pub struct Renderable {
    paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }

    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("Invalid RenderableKind"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, idx: usize) -> &str {
        &self.paths[idx % self.paths.len()]
    }
}

/// Position of the entity in the game world. The z-axis determines whether a renderable entity
/// is drawn onto or below another renderable entity.
pub type Position = mint::Point3<u8>;

/// This component determines the color of a box archetype.
#[derive(PartialEq)]
pub enum BoxColor {
    Blue,
    Red,
}

/// Marker represents a box in sokoban.
pub struct Box {
    pub color: BoxColor,
}

/// Marker represents a box destination in sokoban.
pub struct BoxSpot {
    pub color: BoxColor,
}

/// Marker represents a player in sokoban.
#[derive(Default)]
pub struct Player;

/// Marker represents a wall in sokoban.
#[derive(Default)]
pub struct Wall;

/// A moveable entity can be moved by the player.
#[derive(Default)]
pub struct Movable;

/// A moveable entity can not be moved by the player.
#[derive(Default)]
pub struct Immovable;
