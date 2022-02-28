//! A 2D point representing a location in a results space.
use rand::Rng;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub retain: bool,
}

impl Point {
    /**
    Generate a randomised vector of Points constrained by the current window size

    # Arguments

    * - `window_size` - The size of the current window. This sets the upper limit for the x and y values
    * - `count` - the number of points in the vectored vector

    # Examples

    ```
    fn render(&mut self, e: &piston::Event, args: piston::RenderArgs) {
        if let None = self.points {
            self.points = Some(Point::rnd_vec(args.window_size, 500));
        };
    }
    ```
    */
    pub fn rnd_vec(window_size: [f64; 2], count: usize) -> Vec<Point> {
        (0..count).map(|_| Point::rnd(window_size)).collect()
    }

    /// Generate a single random Point constrained by the the current wondow size
    pub fn rnd(window_size: [f64; 2]) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            retain: true,
            x: rng.gen_range(0..window_size[0] as i32) as f64,
            y: rng.gen_range(0..window_size[1] as i32) as f64,
        }
    }
}
