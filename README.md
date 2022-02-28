# decluster

Iteratively declusters a randomised set of points.

Declustering is the process of removing points in a result space that are considered too close together. This demo crate implements a custom
declustering algorithm that works on an inital set of randomised 2D points finding and replacing clusters with new random points until it settles on the
maximally declustered set, if that is acheivable. As the algorithm works the results are dynamically displayed in a window.

### Note
Depending on your wondow size, the number of points and the minimum distance you specify, it may not be possible to fit all the points in. In this case you will see the algorithm coninue
continue to endlessly seek to distribute the points.

There are interesting balance points where the set is hard to find but possible. For example, with a screen size of [2560, 1440] with
500 points the balance point exists when the minimum distance lies between 65 and 70. That is when you will see the algorithm take its time but eventually settle on a nice, almost grid-like distribution.

[Before](decluster_before.png)

### Examples

Basic Usage:
```
let point_count = 500;
let min_distance = 65.0;

Canvas::new(point_count, min_distance).show();
```
