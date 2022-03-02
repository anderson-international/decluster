/*!
Performs a random search for a viable set of points where each point is separated from all other points by at least the specified minimum distance.
It  works on an inital set of randomised 2D points finding and replacing clusters with new random points until it settles on the viable declustered set -
if that is acheivable given the number of points and, the size of the window and minimum allowable distance.

### Basic Usage
```
use decluster::Canvas;

pub fn main() {
    let point_count = 500;
    let min_distance = 58.0;

    Canvas::new(point_count, min_distance).show();
}

```

### Examples

For more notes and a working example please see the decluster_demo example that accompanies this lib. The source for the demo is listed
under the [examples](https://github.com/anderson-international/decluster/tree/master/examples) directory on GitHub.

To compile and run the example use:

```
> cargo run --example decluster_demo --release
```

Once you get it running then have play with the `point_count` and `min_distance` parameters to find the interesting tipping points where viable sets exist but
are increasingly hard to find.

For example with a screen size of [2560, 1440] and a point count of 500 the balance point exists when the minimum distance lies between 55 and 60.
With these numbers there exists a limited set of viable distributions that manage to fit all the points whilst maintaining the specified minimum distance.
As a result you will see the algorithm take its time before eventually settling on a viable distribution. Increasing the minimum distance just a little from here
will increase the difficulty further until eventually no viable solutions exist and the algorithm will cycle.
*/

extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod decluster;
mod point;

use graphics::{clear, rectangle};
use piston::{RenderArgs, RenderEvent, UpdateEvent};
use piston_window::{color, PistonWindow, WindowSettings};
use point::Point;

/// The drawing surface and piston window used to display the set of points.
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
    * `min_distance` - if a pair of points are closer than the `min_distance`, then one of the points will be removed and replaced with a fresh random point
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
            clear(color::BLACK, gl);
            for point in points {
                rectangle(color::GREEN, [point.x, point.y, 5.0, 5.0], c.transform, gl);
            }
        });
    }
    /**
    Declusters the current set of points then refills the set with new randomised points. This amounts to a random search for a viable set, that is a set of points
    where each individual point is separated by at least the minimum distance.
    */
    fn update(&mut self) {
        if let Some(points) = self.points.as_mut() {
            let len = points.len();
            decluster::decluster(points, self.min_distance);
            let count = len - points.len();
            points.append(&mut Point::rnd_vec(self.window_size, count));
        }
    }
}
