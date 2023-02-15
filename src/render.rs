use mandelbrot::Coord;
use mandelbrot::Fractal;
use image::ImageBuffer;
use image::ImageError;
use image::ImageFormat;
use num_complex::Complex;

// Renders a tile and saves it in the tile cache.
pub fn render(filename: &String, zoom: u8, x: i16, y: i16) -> Result<(), ImageError> {
  // Resolution of each tile.
  let res: u32 = 2048;

  // At zoom 0, the center tile goes from -4.0 to 4.0 in both x and y directions.
  let base: f64 = -4.0;

  // At zoom 0, each tile covers 8.0 units. Range doubles for each zoom level.
  let range: f64 = 8.0 / (2.0f64).powf(zoom as f64);

  let x_min: f64 = base + range*(x as f64);
  let x_max: f64 = base + range*(1.0 + x as f64);
  let y_min: f64 = base + range*(y as f64);
  let y_max: f64 = base + range*(1.0 + y as f64);

  let fract = Fractal {
    x: Coord {
      min: x_min,
      max: x_max,
      samples: res,
    },
    y: Coord {
      min: y_min,
      max: y_max,
      samples: res,
    },
    c: Complex::new(0.0, 0.0),
  };

  let mut imgbuf = ImageBuffer::new(fract.x.samples, fract.y.samples);
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let i = 255 - fract.mandelbrot(x, y);
    *pixel = image::Rgb([i, i, i]);
  }
  imgbuf.save_with_format(filename, ImageFormat::Png)
}
