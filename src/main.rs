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

    // set background image
    draw.texture(&model.background)
        .wh(app.window_rect().wh()) // match window size
        .xy(app.window_rect().xy()); // center

    let win = app.window_rect();

    // render our Model
    model.draw(win, &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
