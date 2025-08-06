use nannou::prelude::*;

pub mod clock;

pub trait Drawable {
    fn draw(&self, bounds: Rect, draw: &Draw);
    fn update(&mut self, _update: &Update) {}
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

    /// Takes rows, cols and slices a Rect up evenly
    ///
    /// ```
    /// use nannou::prelude::*;
    /// use klox::RectUtils;
    ///
    /// let rect = Rect::from_w_h(80.0, 30.0);
    ///
    /// // Slice it up into 8 columns and 3 rows
    /// let grid: [[Rect; 3]; 4] = rect.grid();
    ///
    /// // Row 0
    /// assert_eq!(grid[0][0], Rect::from_corner_points([-40.0, 15.0], [-20.0, 5.0]));
    /// assert_eq!(grid[1][0], Rect::from_corner_points([-20.0, 15.0], [0.0, 5.0]));
    /// assert_eq!(grid[2][0], Rect::from_corner_points([0.0, 15.0], [20.0, 5.0]));
    /// assert_eq!(grid[3][0], Rect::from_corner_points([20.0, 15.0], [40.0, 5.0]));
    /// // Row 1
    /// assert_eq!(grid[0][1], Rect::from_corner_points([-40.0, 5.0], [-20.0, -5.0]));
    /// assert_eq!(grid[1][1], Rect::from_corner_points([-20.0, 5.0], [0.0, -5.0]));
    /// assert_eq!(grid[2][1], Rect::from_corner_points([0.0, 5.0], [20.0, -5.0]));
    /// assert_eq!(grid[3][1], Rect::from_corner_points([20.0, 5.0], [40.0, -5.0]));
    /// // Row 1
    /// assert_eq!(grid[0][2], Rect::from_corner_points([-40.0, -5.0], [-20.0, -15.0]));
    /// assert_eq!(grid[1][2], Rect::from_corner_points([-20.0, -5.0], [0.0, -15.0]));
    /// assert_eq!(grid[2][2], Rect::from_corner_points([0.0, -5.0], [20.0, -15.0]));
    /// assert_eq!(grid[3][2], Rect::from_corner_points([20.0, -5.0], [40.0, -15.0]));
    /// ```
    fn grid<const C: usize, const R: usize>(self) -> [[Self; R]; C];
}

impl RectUtils for Rect {
    fn split_y_axis(self) -> (Self, Self) {
        (
            Rect::from_corners(self.mid_bottom(), self.top_left()),
            Rect::from_corners(self.mid_top(), self.bottom_right()),
        )
    }

    fn grid<const C: usize, const R: usize>(self) -> [[Self; R]; C] {
        let (l, t, w, h) = self.l_t_w_h();
        let col_width = w / C as f32;
        let row_height = h / R as f32;
        core::array::from_fn(|c| {
            let c = c as f32;
            core::array::from_fn(|r| {
                let r = r as f32;
                Rect::from_corner_points(
                    [l + c * col_width, t - r * row_height],
                    [l + (c + 1.0) * col_width, t - (r + 1.0) * row_height],
                )
            })
        })
    }
}
