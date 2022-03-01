# decluster

This demo uses the custom declustering algorithm to perform a random search for a viable set of points where each point is separated from all other points by at least the specified minimum distance. It  works on an inital set of randomised 2D points finding and replacing clusters with new random points until it settles on a viable declustered set.

## Notes
Depending on your window size, the number of points and the minimum distance you specify, it may not be possible to fit all the points in. In this case you will see the algorithm
endlessly seek the best distribution but never settle down.

There are interesting balance points where the set is hard to find but possible. For example with a screen size was [2560, 1440] and a point count of 500 the balance point exists
when the minimum distance lies between 55 and 60.

With this setup there are a limited set of viable distributions that can fit all the points in whilst maintaining the specified distance. As a result you will see the algorithm
take its time before eventually settling on a viable distribution. Increasing the minimum distance just a little from here will increase the difficulty further until eventually
no viable solutions exist and the algorithm will cycle.

##### Before - The initial random distribution of points
![Before](decluster_before.png)
##### After - A viable set of distributed points where each point is separated by at least the minimum distance
![Before](decluster_after.png)

### Basic Usage

```
use decluster::Canvas;

fn main() {
    let point_count = 500;
    let min_distance = 58.0;

    Canvas::new(point_count, min_distance).show();
}

```
