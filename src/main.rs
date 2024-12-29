use dotenv::dotenv;
use env_logger;
use log::info;

pub mod color;
pub mod vec3;
pub mod ray;

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
            
            let pixel_color = color::Color { 
                x: (i as f64)/((image_width - 1) as f64), 
                y: (j as f64)/((image_height-1) as f64), 
                z: 0.0 
            };
            color::write_color(pixel_color);
        }
    }
    
    info!("Done");
}