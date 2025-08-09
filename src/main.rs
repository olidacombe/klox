use klox::Drawable;
use klox::clock::{Model, app};
use nannou::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();
    app().simple_window(view).run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // // get canvas to draw on
    let draw = app.draw();

    let win = app.window_rect();
    let (win_w, win_h) = (win.w(), win.h());

    // --- COVER FIT (fill & crop; preserves aspect ratio) ---
    // Scale so the smaller axis fits, then crop the overflow.
    let sx = win_w / model.background_width;
    let sy = win_h / model.background_height;
    let scale = sx.max(sy); // cover
    let target_w = model.background_width * scale;
    let target_h = model.background_height * scale;

    // set background image
    draw.texture(&model.background)
        .xy(win.xy()) // center in the window
        .w_h(target_w, target_h);

    // render our Model
    model.draw(win, &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
