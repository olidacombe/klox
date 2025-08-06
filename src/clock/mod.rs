use nannou::prelude::*;

use crate::Drawable;

#[derive(Default)]
struct Clocklet {
    a: f64,
    b: f64,
}

impl Drawable for Clocklet {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        draw.ellipse().xy(bounds.xy()).wh(bounds.wh()).color(SALMON);
    }
}

#[derive(Default)]
struct Clock {
    clocklets: [Clocklet; 24],
}

// impl Drawable for Clock {
//     fn draw(&self, bounds: Rect, draw: &Draw) {
//         let
//     }
// }

#[derive(Default)]
pub struct Model {
    clock: Clock,
}

impl Drawable for Model {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let bounds = bounds.pad(25.0);
        draw.ellipse().xy(bounds.xy()).wh(bounds.wh()).color(SALMON);
    }
}

pub fn app() -> nannou::app::Builder<Model> {
    nannou::app(model).event(event)
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}
