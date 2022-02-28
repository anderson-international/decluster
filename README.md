# decluster

Iteratively declusters a randomised set of points.

Declustering is the process of removing points in a result space that are considered too close together. This demo crate implements a custom
declustering algorithm that works on an inital set of randomised 2D points finding and replacing clusters with new random points until it settles on the
maximally declustered set, if that is acheivable. As the algorithm works the results are dynamically displayed in a window.

#### Note
Depending on your window size, the number of points and the minimum distance you specify, it may not be possible to fit all the points in. In this case you will see the algorithm endlessly seek the best distribution but never settle down.

There are interesting balance points where the set is hard to find but possible. In the xample below the screen size was [2560, 1440] and the point count was 1000. In this case the balance point exists when the minimum distance lies between 45 and 50. There are a very limited set of viable distributions that can fit all the points in whilst maintaining the specified distance and so you will see the algorithm take its time before eventually settling on a viable distribution. Increasing the minimum distance just a little from here will increase the difficulty further until eventually no viable solutions exist and the algorithm will cycle.

##### Before - The initial random distribution of points
![Before](decluster_before.png)
##### After - A viable set of distributed points where each point is separated by at least the minimum distance
![Before](decluster_after.png)

### Examples

Basic Usage:
```
let point_count = 1000;
let min_distance = 47.0;

Canvas::new(point_count, min_distance).show();
```
