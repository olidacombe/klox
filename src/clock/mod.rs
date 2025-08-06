use nannou::prelude::*;

use crate::Drawable;

#[derive(Default)]
pub struct Clocklet {
    a: f64,
    b: f64,
}

impl Drawable for Clocklet {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let r = partial_min(bounds.w(), bounds.h());
        draw.ellipse().xy(bounds.xy()).w_h(r, r).color(WHITE);
    }
}

struct Clock {
    /// 8 columns of 3 clocklets
    /// 2 columns form 1 digit
    /// I didn't express in terms of digits, because
    /// general-purpose animations (i.e. not displaying numbers)
    /// will be really cumbersome
    ///
    ///
    /// When I say `digits`, I mean like some kind of
    /// ```
    /// use klox::clock::Clocklet;
    /// struct Digit {
    ///     clocklets: [[Clocklet; 3]; 2]
    /// }
    /// ```
    clocklets: [[Clocklet; 3]; 8],
    padding: f32,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            padding: 100.0,
            clocklets: Default::default(),
        }
    }
}

impl Drawable for Clock {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        self.clocklets[0][0].draw(bounds, draw);
    }
}

pub struct Model {
    padding: f32,
    clock: Clock,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            padding: 10.0,
            clock: Clock::default(),
        }
    }
}

impl Drawable for Model {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let bounds = bounds.pad(self.padding);
        self.clock.draw(bounds, draw);
    }
}

pub fn app() -> nannou::app::Builder<Model> {
    nannou::app(model).event(event)
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}
