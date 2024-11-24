use web_sys::CanvasRenderingContext2d;

use crate::ITERATIONS_PER_FRAME;

pub struct Pendulum {
    /// x position
    x: f64,
    /// y position
    y: f64,
    /// x velocity
    vx: f64,
    /// y velocity
    vy: f64,
    points: Vec<(f64, f64)>,
}

impl Pendulum {
    pub fn new(start: (f64, f64)) -> Self {
        let (x, y) = start;
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            points: Default::default(),
        }
    }
    const FORCE_COEFFICIENT: f64 = 0.1;
    const MASS: f64 = 20.0;
    const D_T: f64 = 1.0 / ITERATIONS_PER_FRAME as f64;
    pub fn tick(&mut self) {
        let force_x = -self.x * Self::FORCE_COEFFICIENT;
        let force_y = -self.y * Self::FORCE_COEFFICIENT;
        let a_x = force_x / Self::MASS;
        let a_y = force_y / Self::MASS;
        let d_vx = a_x * Self::D_T;
        let d_vy = a_y * Self::D_T;
        self.vx += d_vx;
        self.vy += d_vy;
        let d_x = self.vx * Self::D_T;
        let d_y = self.vy * Self::D_T;
        self.x += d_x;
        self.y += d_y;
        self.points.push((self.x, self.y));
    }

    pub fn flush(&mut self, ctx: &CanvasRenderingContext2d) {
        let to_draw = std::mem::take(&mut self.points);
        to_draw.into_iter().enumerate().for_each(|(i, (x, y))| {
            if i == 0 {
                ctx.move_to(x, y)
            } else {
                ctx.line_to(x, y)
            }
        });
        ctx.stroke();
    }
}
