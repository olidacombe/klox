use nannou::image::GenericImageView;
use nannou::prelude::*;
use tracing::debug;

use std::{
    collections::VecDeque,
    f64::consts::TAU,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{Drawable, RectUtils};

struct Digit([Clocklet; 6]);

impl Digit {
    const ZERO: Self = Self([
        Clocklet::TL,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::BR,
    ]);
    const ONE: Self = Self([
        Clocklet::BLANK,
        Clocklet::BLANK,
        Clocklet::BLANK,
        Clocklet::D,
        Clocklet::V,
        Clocklet::U,
    ]);
    const TWO: Self = Self([
        Clocklet::R,
        Clocklet::TL,
        Clocklet::BL,
        Clocklet::TR,
        Clocklet::BR,
        Clocklet::L,
    ]);
    const THREE: Self = Self([
        Clocklet::R,
        Clocklet::R,
        Clocklet::R,
        Clocklet::TR,
        Clocklet::BR,
        Clocklet::BR,
    ]);
    const FOUR: Self = Self([
        Clocklet::D,
        Clocklet::BL,
        Clocklet::BLANK,
        Clocklet::D,
        Clocklet::BR,
        Clocklet::U,
    ]);
    const FIVE: Self = Self([
        Clocklet::TL,
        Clocklet::BL,
        Clocklet::R,
        Clocklet::L,
        Clocklet::TR,
        Clocklet::BR,
    ]);
    const SIX: Self = Self([
        Clocklet::TL,
        Clocklet::V,
        Clocklet::BL,
        Clocklet::L,
        Clocklet::TR,
        Clocklet::BR,
    ]);
    const SEVEN: Self = Self([
        Clocklet::R,
        Clocklet::BLANK,
        Clocklet::BLANK,
        Clocklet::TR,
        Clocklet::V,
        Clocklet::U,
    ]);
    const EIGHT: Self = Self([
        Clocklet::TL,
        Clocklet::BL,
        Clocklet::BL,
        Clocklet::TR,
        Clocklet::BR,
        Clocklet::BR,
    ]);
    const NINE: Self = Self([
        Clocklet::TL,
        Clocklet::BL,
        Clocklet::R,
        Clocklet::TR,
        Clocklet::BR,
        Clocklet::BR,
    ]);
    const BLANK: Self = Self([Clocklet::BLANK; 6]);
}

impl From<u64> for Digit {
    fn from(value: u64) -> Self {
        match value % 10 {
            0 => Digit::ZERO,
            1 => Digit::ONE,
            2 => Digit::TWO,
            3 => Digit::THREE,
            4 => Digit::FOUR,
            5 => Digit::FIVE,
            6 => Digit::SIX,
            7 => Digit::SEVEN,
            8 => Digit::EIGHT,
            9 => Digit::NINE,
            _ => unreachable!(),
        }
    }
}

impl<'a> IntoIterator for &'a Digit {
    type Item = &'a Clocklet;
    type IntoIter = std::slice::Iter<'a, Clocklet>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
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
    pub const L: Clocklet = Clocklet::from_turns(0.75, 0.75);
    pub const R: Clocklet = Clocklet::from_turns(0.25, 0.25);
    pub const TL: Clocklet = Clocklet::from_turns(0.5, 0.25);
    pub const TR: Clocklet = Clocklet::from_turns(0.5, 0.75);
    pub const V: Clocklet = Clocklet::from_turns(0.0, 0.5);
    pub const U: Clocklet = Clocklet::from_turns(0.0, 0.0);
    pub const D: Clocklet = Clocklet::from_turns(0.5, 0.5);
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

impl Default for Lifespan {
    fn default() -> Self {
        // Basically "immediately"
        Self::Pending(Duration::default())
    }
}

#[derive(Default)]
struct ClockTarget {
    clocklets: [[Clocklet; 3]; 8],
    extra_turns: Option<[[f64; 3]; 8]>,
    pub lifespan: Lifespan,
}

impl ClockTarget {
    pub fn from_time(time: &Duration, lifespan: Lifespan) -> Self {
        let mut me = Self::default();

        let time = time.as_secs() / 60;
        let mut mins = time % 60;
        let mut hours = (time / 60) % 24;
        debug!("got time {hours}:{mins}");
        me.set_digit(&mins.into(), 3);
        mins /= 10;
        me.set_digit(&mins.into(), 2);
        me.set_digit(&hours.into(), 1);
        hours /= 10;
        me.set_digit(&hours.into(), 0);
        me.lifespan = lifespan;
        me
    }

    pub fn set_digit(&mut self, digit: &Digit, position: usize) {
        let position = (position % 4) * 2;

        let scope = &mut self.clocklets[position..(position + 2)];

        let clocklets = scope.iter_mut().flatten();

        for (src, dst) in digit.into_iter().zip(clocklets) {
            *dst = *src;
        }
    }

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
            clocklets: Default::default(),
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

    pub fn clobber_targets(&mut self, target: ClockTarget) {
        debug!("🔨 Clobbering clock with single target 🤷");
        if let Some(ClockTarget {
            lifespan: Lifespan::Active { .. },
            ..
        }) = self.targets.front()
        {
            self.clocklets = self.interpolated_clocklets();
        }
        self.targets = [target].into();
    }

    pub fn lerp(&self, target: &ClockTarget) -> [[Clocklet; 3]; 8] {
        let progress = target.progress();

        core::array::from_fn(|col| {
            core::array::from_fn(|row| {
                self.clocklets[col][row]
                    + (target.clocklets[col][row] - self.clocklets[col][row]) * progress
            })
        })
    }

    fn as_target(&self) -> ClockTarget {
        ClockTarget {
            clocklets: self.clocklets,
            extra_turns: None,
            // lifespan: Lifespan::default(),
            lifespan: Lifespan::Pending(Duration::from_millis(1000)),
        }
    }

    /// Get a ClockTarget from Clock by replacing 6 clocklets with a given digit.
    /// Useful for working on digit definitions
    pub fn target_digit(&mut self, digit: &Digit, position: usize) {
        let mut target = self.as_target();
        target.set_digit(digit, position);
        self.push_target(target);
    }

    fn interpolated_clocklets(&self) -> [[Clocklet; 3]; 8] {
        // FIXME more implicit cloning
        self.targets
            .front()
            .map(|targets| self.lerp(targets))
            .unwrap_or(self.clocklets)
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

        let clocklets = self.interpolated_clocklets();

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
                self.clocklets = updated.clocklets;
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

#[derive(Default)]
struct TriggerTime(bool);

impl TriggerTime {
    const LEAD_TIME_SECONDS: u64 = 5;
    const TRIGGER_TIME_SECONDS: u64 = 60 - Self::LEAD_TIME_SECONDS;

    pub fn trigger(&mut self) -> Option<ClockTarget> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let seconds = now.as_secs() % 60;

        // We have already triggered, check if we should re-arm
        if self.0 && seconds < Self::TRIGGER_TIME_SECONDS {
            self.0 = false;
        } else if !self.0 && seconds >= Self::TRIGGER_TIME_SECONDS {
            self.0 = true;

            let target = ClockTarget::from_time(
                &(now + Duration::from_secs(Self::LEAD_TIME_SECONDS)),
                Lifespan::from_millis(Self::LEAD_TIME_SECONDS * 1000),
            );
            return Some(target);
        }

        None
    }
}

pub struct Model {
    padding: f32,
    clock: Clock,
    debug_digit: usize,
    trigger_time: TriggerTime,
    pub background: wgpu::Texture,
    pub background_width: f32,
    pub background_height: f32,
}

impl Model {
    pub fn scramble_millis(&mut self, millis: u64) {
        self.clock.push_target(ClockTarget::random_millis(millis));
    }

    fn new(app: &App) -> Self {
        // Put your PNG in ./assets/background.png
        let assets = app.assets_path().expect("assets dir");
        let img_path = assets.join("background.png");

        // Load once via image to get dimensions, then create a texture.
        let img = nannou::image::open(&img_path).expect("open background.png");
        let (w, h) = img.dimensions();
        let background = wgpu::Texture::from_image(app, &img);

        Self {
            padding: 10.0,
            clock: Default::default(),
            debug_digit: 0,
            background,
            background_width: w as f32,
            background_height: h as f32,
            trigger_time: Default::default(),
        }
    }
}

impl Drawable for Model {
    fn draw(&self, bounds: Rect, draw: &Draw) {
        let (w, h) = bounds.w_h();
        let bounds = Rect::from_w_h(clamp_max(w, h * 8.0 / 3.0), clamp_max(h, w * 3.0 / 8.0));
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

fn model(app: &App) -> Model {
    Model::new(app)
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
            Key::Space => {
                model.clock.target_digit(&Digit::BLANK, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key0 => {
                model.clock.target_digit(&Digit::ZERO, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key1 => {
                model.clock.target_digit(&Digit::ONE, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key2 => {
                model.clock.target_digit(&Digit::TWO, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key3 => {
                model.clock.target_digit(&Digit::THREE, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key4 => {
                model.clock.target_digit(&Digit::FOUR, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key5 => {
                model.clock.target_digit(&Digit::FIVE, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key6 => {
                model.clock.target_digit(&Digit::SIX, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key7 => {
                model.clock.target_digit(&Digit::SEVEN, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key8 => {
                model.clock.target_digit(&Digit::EIGHT, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            Key::Key9 => {
                model.clock.target_digit(&Digit::NINE, model.debug_digit);
                model.debug_digit = (model.debug_digit + 1) % 4;
            }
            _ => {}
        },
        Event::Update(ref update) => {
            if let Some(time_target) = model.trigger_time.trigger() {
                model.clock.clobber_targets(time_target);
            }
            model.update(update);
        }
        _ => {}
    }
}
