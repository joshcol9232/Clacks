use ggez::graphics::{self, Mesh, DrawMode, DrawParam, Rect};
use ggez::{Context, GameResult};

use crate::FLOOR_POS;

#[derive(Default)]
pub struct Block {
    pub pos: f64,       // One dimensional
    min_pos: f64,
    pub vel: f64,
    pub mass: f64,
    pub size: f64,
}

impl Block {
    pub fn new(pos: f64, vel: f64, mass: f64, size: f64, min_pos: f64) -> Block {
        Block {
            pos,
            min_pos,
            vel,
            mass,
            size
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let sqr = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, self.size as f32, self.size as f32),
            [1.0, 0.0, 0.1, 1.0].into()
        )?;
        
        graphics::draw(ctx, &sqr,
            DrawParam::default()
                .dest([
                    self.pos as f32,
                    FLOOR_POS - self.size as f32
                ])
        )
    }

    #[inline]
    pub fn update_pos(&mut self, dt: f64) {
        self.pos += self.vel * dt;
        self.pos = self.pos.max(self.min_pos);
    }

    pub fn bounce(&mut self, other: &Self) -> f64 {
        let total_mass = self.mass + other.mass;
        let mut new_vel = ((self.mass-other.mass)/total_mass) * self.vel;
        new_vel += (2.0 * other.mass / total_mass) * other.vel;
        new_vel
    }

    // Only works on left side of block
    pub fn hit_wall(&mut self, wall_pos: f64) -> bool {
        if self.pos <= wall_pos {
            self.vel = -self.vel;
            return true
        }
        return false
    }
}