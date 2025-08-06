use nannou::prelude::*;

pub struct Model {}

impl Model {
    pub fn draw(&self, bounds: Rect, draw: &Draw) {
        let bounds = bounds.pad(25.0);
        draw.ellipse().xy(bounds.xy()).wh(bounds.wh()).color(SALMON);
    }
}

pub fn app() -> nannou::app::Builder<Model> {
    nannou::app(model).event(event)
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}
