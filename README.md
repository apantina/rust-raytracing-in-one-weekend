
#  Ray Tracing in One Weekend - Rust

A Rust implementation of Peter Shirley-s amazing tutorial - [link](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

This was done for the sole reason of learning Rust via an actual project, so don't expect the code to be 100% idiomatic (or good, for that matter :)).
Apart from the tutorial itself, parallel processing was also added using the `rayon` crate.

## Building and running
Build the program using `cargo build`.

Then you can run the ray tracer, which outputs the image in PPM format to the standard output.
You can redirect this image to a file: 

```bash
./target/debug/raytracing-one-weekend > image.ppm
```

Rendering the full image may take a while, took about 3 minutes on my poor little laptop :D
The `max_depth` and `samples_per_pixel` parameters to make the image more detailed, requiring more processing time.

Rendering of the final image (using parallel processing via `rayon`):
```bash
❯ time ./target/debug/rust-raytracing-in-one-weekend > image.ppm && xdg-open image.ppm
./target/debug/rust-raytracing-in-one-weekend > image.ppm  1623,51s user 2,42s system 732% cpu 3:41,87 total
```
CPU: Intel i7-4700MQ (8) @ 3.400GHz

![image](./blob/master/image.ppm?raw=true)