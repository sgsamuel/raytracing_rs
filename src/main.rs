use dotenv::dotenv;
use log::info;
use env_logger;

#[derive(Debug)]
pub struct Config {
    pub rust_log: String,
}

impl Config {
    pub fn init() -> Config {
        Config {
            rust_log: std::env::var("RUST_LOG")
                .expect("RUST_LOG must be specified"),
        }
    }
}

fn main() {
  dotenv().ok();
  env_logger::init();

  let image_width: u32 = 256;
  let image_height: u32 = 256;
  
  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  for j in 0..image_height {
    for i in 0..image_width {
        info!("Scanlines remaining: {}", image_height - j);
        
        let r = i as f64 / (image_width - 1) as f64;
        let g = j as f64 / (image_height - 1) as f64;
        let b = 0.0;

        let ir = (255.999 * r) as i32;
        let ig = (255.999 * g) as i32;
        let ib = (255.999 * b) as i32;

        println!("{} {} {}", ir, ig, ib);
    }
}
}