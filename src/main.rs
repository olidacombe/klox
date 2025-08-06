use klox::Drawable;
use klox::clock::{Model, app};
use nannou::prelude::*;

fn main() {
    app().simple_window(view).run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    let win = app.window_rect();

    // render our Model
    model.draw(win, &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
