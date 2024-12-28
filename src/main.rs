fn main() {
  let image_width: u32 = 256;
  let image_height: u32 = 256;
  
  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  for j in 0..image_height {
    for i in 0..image_width {
        // Calculate RGB values
        let r = i as f64 / (image_width - 1) as f64;
        let g = j as f64 / (image_height - 1) as f64;
        let b = 0.0;

        // Convert to integer values in the range [0, 255]
        let ir = (255.999 * r) as i32;
        let ig = (255.999 * g) as i32;
        let ib = (255.999 * b) as i32;

        // Output the RGB values
        println!("{} {} {}", ir, ig, ib);
    }
}
}