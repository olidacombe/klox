use nannou::prelude::*;

use std::{f64::consts::TAU, time::Duration};

use crate::{Drawable, RectUtils};

pub struct Clocklet {
    /// hour hand expressed as fraction of a full turn
    hour_hand_turns: f64,
    /// minute hand expressed as fraction of a full turn
    minute_hand_turns: f64,
}

impl Default for Clocklet {
    fn default() -> Self {
        Self {
            hour_hand_turns: random_f64(),
            minute_hand_turns: random_f64(),
        }
    }
}

impl Clocklet {
    /// Returns normal vectors corresponding to (a, b)
    fn vectors(&self) -> (Point2, Point2) {
        let a_rad = TAU * self.hour_hand_turns;
        let b_rad = TAU * self.minute_hand_turns;
        (
            Point2::new(a_rad.sin() as f32, a_rad.cos() as f32),
            Point2::new(b_rad.sin() as f32, b_rad.cos() as f32),
        )
    }
}

impl Drawable for Clocklet {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let d = partial_min(bounds.w(), bounds.h());
        let o = bounds.xy();
        draw.ellipse().xy(o).w_h(d, d).color(WHITE);
        let r = d * 0.45;
        let (hours, mins) = self.vectors();
        draw.line().weight(5.0).start(o).end(o + r * hours);
        draw.line().weight(5.0).start(o).end(o + r * mins);
    }
}

enum Deadline {
    Absolute(Duration),
    Relative(Duration),
}

impl Deadline {
    pub fn from_millis(millis: u64) -> Self {
        Self::Relative(Duration::from_millis(millis))
    }

    pub fn absolute(self, now: Duration) -> Self {
        match self {
            Self::Absolute(t) => self,
            Self::Relative(t) => Self::Absolute(now + t),
        }
    }
}

enum Lifespan {
    Pending(Deadline),
    Active { start: Duration, deadline: Deadline },
    Finished,
}

impl Lifespan {
    pub fn from_millis(millis: u64) -> Self {
        Self::Pending(Deadline::from_millis(millis))
    }
}

struct ClockTarget {
    target: [[Clocklet; 3]; 8],
    extra_turns: [[f64; 3]; 8],
    lifespan: Lifespan,
}

impl ClockTarget {
    pub fn random_milles(millis: u64) -> Self {
        Self {
            target: Default::default(),
            extra_turns: [[3.0; 3]; 8],
            lifespan: Lifespan::from_millis(millis),
        }
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
    /// Stack of animation targets to process
    targets: Vec<ClockTarget>,
    padding: f32,
}

impl Clock {
    pub fn push_target(&mut self, target: ClockTarget) {
        self.targets.push(target);
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            padding: 8.0,
            clocklets: Default::default(),
            targets: Default::default(),
        }
    }
}

impl Drawable for Clock {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let grid: [[Rect; 3]; 8] = bounds.grid();
        for (i, col) in grid.into_iter().enumerate() {
            for (j, rect) in col.into_iter().enumerate() {
                self.clocklets[i][j].draw(rect.pad(self.padding), draw);
            }
        }
    }
}

pub struct Model {
    padding: f32,
    clock: Clock,
}

impl Model {
    pub fn scramble_millis(&mut self, millis: u64) {
        self.clock.push_target(ClockTarget::random_milles(millis));
    }
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

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(WindowEvent::KeyPressed(key)),
        ..
    } = event
    {
        match key {
            Key::Q => {
                app.quit();
            }
            Key::R => {
                model.scramble_millis(3);
            }
            _ => {}
        }
    }
}
