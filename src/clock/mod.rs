use nannou::prelude::*;
use tracing::debug;

use std::{
    collections::VecDeque,
    f64::consts::TAU,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    time::Duration,
};

use crate::{Drawable, RectUtils};

struct Digit([Clocklet; 6]);

impl Digit {
    const ZERO: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const ONE: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const TWO: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const THREE: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const FOUR: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const FIVE: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const SIX: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const SEVEN: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const EIGHT: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const NINE: Self = Self([
        Clocklet::TL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::BR,
    ]);
    const BLANK: Self = Self([Clocklet::BLANK; 6]);
}

#[derive(Clone, Copy)]
pub struct Clocklet {
    /// hour hand expressed as fraction of a full turn
    hour_hand_turns: f64,
    /// minute hand expressed as fraction of a full turn
    minute_hand_turns: f64,
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

    pub const fn from_turns(hour_hand_turns: f64, minute_hand_turns: f64) -> Self {
        Self {
            hour_hand_turns,
            minute_hand_turns,
        }
    }

    pub const BL: Clocklet = Clocklet::from_turns(0.0, 0.25);
    pub const BLANK: Clocklet = Clocklet::from_turns(0.625, 0.625);
    pub const BR: Clocklet = Clocklet::from_turns(0.0, 0.75);
    pub const H: Clocklet = Clocklet::from_turns(0.25, 0.75);
    pub const TL: Clocklet = Clocklet::from_turns(0.25, 0.5);
    pub const TR: Clocklet = Clocklet::from_turns(0.5, 0.75);
    pub const V: Clocklet = Clocklet::from_turns(0.0, 0.5);
}

impl Add for Clocklet {
    type Output = Clocklet;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            hour_hand_turns: self.hour_hand_turns + rhs.hour_hand_turns,
            minute_hand_turns: self.minute_hand_turns + rhs.minute_hand_turns,
        }
    }
}

impl Sub for Clocklet {
    type Output = Clocklet;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            hour_hand_turns: self.hour_hand_turns - rhs.hour_hand_turns,
            minute_hand_turns: self.minute_hand_turns - rhs.minute_hand_turns,
        }
    }
}

impl Mul<f64> for Clocklet {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.hour_hand_turns *= rhs;
        self.minute_hand_turns *= rhs;
        self
    }
}

impl AddAssign for Clocklet {
    fn add_assign(&mut self, rhs: Self) {
        self.hour_hand_turns += rhs.hour_hand_turns;
        self.minute_hand_turns += rhs.minute_hand_turns;
    }
}

impl SubAssign<f64> for Clocklet {
    fn sub_assign(&mut self, rhs: f64) {
        self.hour_hand_turns -= rhs;
        self.minute_hand_turns -= rhs;
    }
}

impl Default for Clocklet {
    fn default() -> Self {
        Self {
            hour_hand_turns: random_f64(),
            minute_hand_turns: random_f64(),
        }
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

enum Lifespan {
    Pending(Duration),
    Active {
        start: Duration,
        current: Duration,
        deadline: Duration,
    },
    Finished,
}

impl Lifespan {
    pub fn from_millis(millis: u64) -> Self {
        Self::Pending(Duration::from_millis(millis))
    }

    pub fn update(mut self, update: &Update) -> Self {
        if let Lifespan::Pending(deadline) = self {
            return Self::Active {
                start: update.since_start,
                current: update.since_start,
                deadline: update.since_start + deadline,
            };
        }
        if let Lifespan::Active {
            ref deadline,
            ref mut current,
            ..
        } = self
        {
            if *deadline < update.since_start {
                debug!(
                    "{deadline:?} passed ({:?}), Lifespan -> Finished",
                    update.since_start
                );
                return Self::Finished;
            }
            *current = update.since_start;
        }
        self
    }
}

struct ClockTarget {
    target: [[Clocklet; 3]; 8],
    extra_turns: Option<[[f64; 3]; 8]>,
    pub lifespan: Lifespan,
}

impl ClockTarget {
    pub fn progress(&self) -> f64 {
        match self.lifespan {
            Lifespan::Finished => 1.0,
            Lifespan::Pending(_) => 0.0,
            Lifespan::Active {
                start,
                current,
                deadline,
            } => {
                let total = (deadline - start).as_secs_f64();
                if total == 0.0 {
                    1.0
                } else {
                    (current - start).as_secs_f64() / total
                }
            }
        }
    }

    pub fn random_millis(millis: u64) -> Self {
        Self {
            target: Default::default(),
            extra_turns: Some([[3.0; 3]; 8]),
            lifespan: Lifespan::from_millis(millis),
        }
    }

    /// Return updated target and extra turns
    pub fn update(mut self, update: &Update) -> (Self, Option<[[f64; 3]; 8]>) {
        self.lifespan = self.lifespan.update(update);
        let extra_turns = self.extra_turns.take();
        (self, extra_turns)
    }

    pub fn is_finished(&self) -> bool {
        if let Lifespan::Finished = self.lifespan {
            return true;
        }
        false
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
    /// Queue of animation targets to process
    targets: VecDeque<ClockTarget>,
    padding: f32,
}

impl Clock {
    pub fn push_target(&mut self, target: ClockTarget) {
        self.targets.push_back(target);
    }

    pub fn lerp(&self, target: &ClockTarget) -> [[Clocklet; 3]; 8] {
        let progress = target.progress();

        core::array::from_fn(|col| {
            core::array::from_fn(|row| {
                self.clocklets[col][row]
                    + (target.target[col][row] - self.clocklets[col][row]) * progress
            })
        })
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

        // FIXME more implicit cloning
        let clocklets = self
            .targets
            .front()
            .map(|targets| self.lerp(targets))
            .unwrap_or(self.clocklets);

        for (i, col) in grid.into_iter().enumerate() {
            for (j, rect) in col.into_iter().enumerate() {
                clocklets[i][j].draw(rect.pad(self.padding), draw);
            }
        }
    }

    fn update(&mut self, update: &Update) {
        while let Some(target) = self.targets.pop_front() {
            let (updated, extra_turns) = target.update(update);
            if updated.is_finished() {
                self.clocklets = updated.target;
                continue;
            }
            if let Some(extra_turns) = extra_turns {
                *self -= extra_turns;
            }
            self.targets.push_front(updated);
            break;
        }
    }
}

impl SubAssign<[[f64; 3]; 8]> for Clock {
    fn sub_assign(&mut self, rhs: [[f64; 3]; 8]) {
        for (i, col) in self.clocklets.iter_mut().enumerate() {
            for (j, clocklet) in col.iter_mut().enumerate() {
                *clocklet -= rhs[i][j];
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
        self.clock.push_target(ClockTarget::random_millis(millis));
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

    fn update(&mut self, update: &Update) {
        self.clock.update(update);
    }
}

pub fn app() -> nannou::app::Builder<Model> {
    nannou::app(model).event(event)
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(key)),
            ..
        } => match key {
            Key::Q => {
                app.quit();
            }
            Key::R => {
                model.scramble_millis(3000);
            }
            _ => {}
        },
        Event::Update(ref update) => {
            model.update(update);
        }
        _ => {}
    }
}
