//! The drawing surface and piston window used to display the set of points.
use crate::{constants, decluster::decluster, point::Point};
use graphics::{clear, rectangle};
use piston::{RenderArgs, RenderEvent, UpdateEvent};
use piston_window::{PistonWindow, WindowSettings};

pub struct Canvas {
    window: PistonWindow,
    points: Option<Vec<Point>>,
    point_count: usize,
    min_distance: f64,
    window_size: [f64; 2],
}

impl Canvas {
    /**
    Returns a new canvas

    # Arguments

    * `point_count` - the number of points in the set
    * `min_distance` - if a pair of points are closer than the `min_distance`, then one of the points will be removed and replaced with a fresh random [point](crate::point::Point)
    */
    pub fn new(point_count: usize, min_distance: f64) -> Self {
        let window: PistonWindow = WindowSettings::new("decluster", [0, 0])
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        Self {
            window,
            points: None,
            point_count,
            min_distance,
            window_size: [0.0, 0.0],
        }
    }

    /**
    Begins the decluster loop by listening for render and update window events.
    */
    pub fn show(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(args) = e.render_args() {
                self.render(&e, args);
            }

            if let Some(_) = e.update_args() {
                self.update();
            }
        }
    }
    /**
    Lazy initialises the the set of points to random locations. This needs to be delayed until the first render so the full-size window dimensions can be
    can be used to constrain the random locations. This window size only become available when the window is first rendered.
    */
    fn render(&mut self, e: &piston::Event, args: RenderArgs) {
        if let None = self.points {
            self.points = Some(Point::rnd_vec(args.window_size, self.point_count));
            self.window_size = args.window_size;
        };

        let points = self.points.as_mut().unwrap();
        self.window.draw_2d(e, |c, gl, _| {
            clear(constants::BLACK, gl);
            for point in points {
                rectangle(
                    constants::GREEN,
                    [point.x, point.y, 5.0, 5.0],
                    c.transform,
                    gl,
                );
            }
        });
    }
    /**
    Declusters the current set of points then refills the set with new randomised points. This amounts to a random search for a viable set, that is a set of points
    where each individual point is seprated by at least the minimum distance.
    */
    fn update(&mut self) {
        if let Some(points) = self.points.as_mut() {
            let len = points.len();
            decluster(points, self.min_distance);
            let count = len - points.len();
            points.append(&mut Point::rnd_vec(self.window_size, count));
        }
    }
}
