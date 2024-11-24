use std::f64::consts::E;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::WINDOW_WIDTH;

pub struct Harmonograph {
    x: Pendulum,
    y: Pendulum,
    points: Vec<(f64, f64)>,
}

impl Harmonograph {
    pub fn new() -> Self {
        Self {
            x: Pendulum { t: 0.0 },
            y: Pendulum { t: 1.0 },
            points: Default::default(),
        }
    }

    pub fn tick(&mut self) {
        let x = self.x.tick();
        let y = self.y.tick();
        self.points.push((x, y));
    }

    const POSITION_OFFSET: f64 = WINDOW_WIDTH / 2.0;
    pub fn flush(&mut self, ctx: &CanvasRenderingContext2d) {
        //ctx.clear_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        ctx.set_fill_style(&JsValue::from_str("green"));
        ctx.set_stroke_style(&JsValue::from_str("#000000"));
        ctx.set_line_width(5.0);
        let to_draw = std::mem::take(&mut self.points);
        to_draw
            .into_iter()
            .for_each(|(x, y)| ctx.line_to(x + Self::POSITION_OFFSET, y + Self::POSITION_OFFSET));
        ctx.stroke();
    }
}

struct Pendulum {
    t: f64,
}

impl Pendulum {
    const AMPLITUDE_1: f64 = 100.0;
    const AMPLITUDE_2: f64 = 500.0;
    const FREQUENCY_1: f64 = 1.0;
    const FREQUENCY_2: f64 = 1.0;
    const DAMPING_1: f64 = 0.01;
    const DAMPING_2: f64 = 0.01;
    const PHASE_1: f64 = 1.0;
    const PHASE_2: f64 = 1.0;
    const D_T: f64 = 0.01;

    pub fn tick(&mut self) -> f64 {
        self.t += Self::D_T;
        self.formula(
            Self::AMPLITUDE_1,
            Self::FREQUENCY_1,
            Self::DAMPING_1,
            Self::PHASE_1,
        ) + self.formula(
            Self::AMPLITUDE_2,
            Self::FREQUENCY_2,
            Self::DAMPING_2,
            Self::PHASE_2,
        )
    }

    fn formula(&self, a: f64, f: f64, d: f64, p: f64) -> f64 {
        (self.t * f + p).sin() * a * E.powf(-d * self.t)
    }
}
