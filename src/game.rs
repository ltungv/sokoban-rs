use ggez::event;
use ggez::graphics::{self, Drawable};
use ggez::mint;
use legion::query::IntoQuery;

use crate::components;
use crate::entities;

pub const TILE_WIDTH: f32 = 48.0;
pub const TILE_HEIGHT: f32 = 48.0;

pub struct Game {
    world: legion::World,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context, map_str: &str) -> ggez::GameResult<Self> {
        let mut world = legion::World::default();
        load_map(ctx, &mut world, map_str)?;
        Ok(Self { world })
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let mut query = <(&components::Position, &components::Renderable)>::query();
        let mut render_data: Vec<(&components::Position, &components::Renderable)> =
            query.iter(&self.world).collect();
        render_data.sort_by_key(|&k| k.0.z);
        for (position, renderable) in render_data {
            let screen_dest = mint::Point2 {
                x: position.x as f32 * TILE_WIDTH,
                y: position.y as f32 * TILE_HEIGHT,
            };

            let mut draw_params = graphics::DrawParam::default().dest(screen_dest);
            if let Some(renderable_dims) = renderable.dimensions(ctx) {
                draw_params = draw_params.scale(mint::Vector2 {
                    x: TILE_WIDTH / renderable_dims.w,
                    y: TILE_HEIGHT / renderable_dims.h,
                });
            }

            graphics::draw(ctx, renderable, draw_params)?;
        }

        graphics::present(ctx)
    }
}

fn load_map(ctx: &mut ggez::Context, world: &mut legion::World, map_str: &str) -> ggez::GameResult {
    let rows: Vec<&str> = map_str.split('\n').map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let cols: Vec<&str> = row.split(' ').collect();
        for (x, col) in cols.iter().enumerate() {
            let position = components::Position {
                x: x as u8,
                y: y as u8,
                z: 0, // this will be modified when the entity is created
            };

            match *col {
                "P" => {
                    let _ = entities::create_floor(ctx, world, position)?;
                    let _ = entities::create_player(ctx, world, position)?;
                }
                "B" => {
                    let _ = entities::create_floor(ctx, world, position)?;
                    let _ = entities::create_box(ctx, world, position)?;
                }
                "W" => {
                    let _ = entities::create_floor(ctx, world, position)?;
                    let _ = entities::create_wall(ctx, world, position)?;
                }
                "S" => {
                    let _ = entities::create_floor(ctx, world, position)?;
                    let _ = entities::create_box_spot(ctx, world, position)?;
                }
                "." => {
                    let _ = entities::create_floor(ctx, world, position)?;
                }
                "N" => {}
                c => panic!("Invalid map item {}", c),
            }
        }
    }
    Ok(())
}
