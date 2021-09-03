# Image Graph

## Introduction
![Sample Output](samples/letter_p.png?raw=true "A graph of a block letter p.")

![Sample Output 2](samples/test.png?raw=true "A (bad) graph of some objects on a table.")

This is a computer vision experiment. It reads an image one pixel at a time and compares each pixel to surrounding pixels. The Pythagorean distance in the dimensions x, y, red, green, and blue is used to determine how close a pair of pixels are. 

Each pixel is given a node on a graph. If two pixels are sufficiently close, an edge is drawn between them.

The resulting graph represents interesting information about the input image, potentially for use in character and object recognition. In practice, this is not useful yet.

Current version divides graph into isolated subgraphs to limit graph size. Subgraphs are stored in the out/ directory, in the .dot format.
## Usage

``` cargo run -- --file path/to/file ```

Arguments:

``` --file / -f ```

Input image

``` --range / -r ```

The search radius in which to measure distance. Higher values increase time cubically, and, depending on other settings, there will be fewer matches at distance, so use sparingly. 

With a range of 2, the following pixels will be considered:

```
[1, 1, 1, 1, 1,
 1, 1, 1, 1, 1,
 1, 1, X, 1, 1
 1, 1, 1, 1, 1
 1, 1, 1, 1, 1]
```

Defaults to 5 (uint 32).

``` --threshhold / -t ```

The maximum distance [sqrt(x^2 + y^2 + r^2 + g^2 + b^2] at which an edge will be drawn.

Defaults to 30.0 (float 64)

``` --space / -s; --color / -c```

Multiplicative factors to apply in colorspace or pixel distance. These are useful because the x,y domain is typically smaller than the r,g,b domain and color will otherwise dominate the total distance. 

Available as arguments because of variations in input images, but comfortable defaults are -s 1 -c 10 (i64).

## Future 
The graphs output by the current version are sometimes meaningful. Further work will be required to extract that meaning, by reducing graph density and shortening chains. Currently progress is stalled, because outputs lack orientation which is critical for distinguishing between, eg., a p and a q. I have not arrived at a good solution to this problem. It would be good to adapt to color ranges of input images as well.