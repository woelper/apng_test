This example uses code from https://docs.rs/zune-png/latest/zune_png/fn.post_process_image.html to demonstrate issues in zune_png's apng example.


Please run:

`cargo run -- ./Animated_PNG_example_bouncing_beach_ball.png`

Frames 2 and 19 have artifacts.

Animated_PNG_example_bouncing_beach_ball.png taken from https://commons.wikimedia.org/wiki/File:Animated_PNG_example_bouncing_beach_ball.png, public domain



![](out/2.png "Frame 2")
![](out/19.png "Frame 19")


This is more apparent in the "clock" example, which can be optained by running `wget https://davidmz.github.io/apng-canvas/images/clock.png` and running `cargo run -- ./clock.png`