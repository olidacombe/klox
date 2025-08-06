use nannou::prelude::*;

pub mod clock;

pub trait Drawable {
    fn draw(&self, bounds: Rect, draw: &Draw);
}

pub trait RectUtils: Sized + Copy {
    /// Takes a and returns 2 sub-Rects split down the y-axis
    /// ```
    /// use nannou::prelude::*;
    /// use klox::RectUtils;
    ///
    /// let rect = Rect::from_w_h(100.0, 100.0);
    /// let (left, right) = rect.split_y_axis();
    ///
    /// assert_eq!(left, Rect::from_corner_points([-50.0, 50.0], [0.0, -50.0]));
    /// assert_eq!(right, Rect::from_corner_points([0.0, 50.0], [50.0, -50.0]));
    /// ```
    fn split_y_axis(self) -> (Self, Self);
}

impl RectUtils for Rect {
    fn split_y_axis(self) -> (Self, Self) {
        (
            Rect::from_corners(self.mid_bottom(), self.top_left()),
            Rect::from_corners(self.mid_top(), self.bottom_right()),
        )
    }
}
