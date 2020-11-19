use ggez::{graphics, mint};

/// Path to the image that represents the entity
pub enum Renderable {
    Image(graphics::Image),
}

impl graphics::Drawable for Renderable {
    /// Draws the drawable onto the rendering target.
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult {
        match self {
            Renderable::Image(image) => image.draw(ctx, param),
        }
    }

    /// Returns a bounding box in the form of a `Rect`.
    ///
    /// It returns `Option` because some `Drawable`s may have no bounding box
    /// (an empty `SpriteBatch` for example).
    fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<graphics::Rect> {
        match self {
            Renderable::Image(image) => Some(image.dimensions()),
        }
    }

    /// Sets the blend mode to be used when drawing this drawable.
    /// This overrides the general [`graphics::set_blend_mode()`](fn.set_blend_mode.html).
    /// If `None` is set, defers to the blend mode set by
    /// `graphics::set_blend_mode()`.
    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        match self {
            Renderable::Image(image) => image.set_blend_mode(mode),
        }
    }

    /// Gets the blend mode to be used when drawing this drawable.
    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        match self {
            Renderable::Image(image) => image.blend_mode(),
        }
    }
}

/// Position of the entity on the game's grid
pub type Position = mint::Point3<u8>;

/// Marks an entity as a player
#[derive(Default)]
pub struct Player;

/// Marks an entity as a box
#[derive(Default)]
pub struct Box;

/// Marks an entity as a wall
#[derive(Default)]
pub struct Wall;

/// Marks an entity as a location where a box can be put into
#[derive(Default)]
pub struct BoxSpot;

#[derive(Default)]
pub struct Moveable;

#[derive(Default)]
pub struct Immovable;
