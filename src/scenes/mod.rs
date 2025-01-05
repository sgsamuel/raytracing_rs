use std::path::Path;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::perlin::PerlinTexture;
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{Checker, Image, Noise};
use crate::utilities;
use crate::vec3::{Point3f, Vec3f};

#[allow(dead_code)]
pub fn simple_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let material_ground: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<Dielectric>   = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<Dielectric> = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Arc<Metal>       = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.2));

    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(0.0,-100.5,-1.0), 100.0, material_ground)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(0.0,0.0,-1.2), 0.5, material_center)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(-1.0,0.0,-1.0), 0.5, material_left)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(-1.0,0.0,-1.0), 0.4, material_bubble)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(1.0,0.0,-1.0), 0.5, material_right)));


    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(-2.0, 2.0, 1.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, -1.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 10.0;
    let focus_dist: f64         = 3.4;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn bouncing_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let ground_material : Arc<Lambertian> = Arc::new(
            Lambertian::from_color(&Color::new(0.5, 0.5, 0.5)
        )
    );
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,-1000.0,0.0), 
                1000.0, 
                ground_material
            )
        )
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = utilities::random();
            let center: Point3f = Point3f::new(
                a as f64 + 0.9 * utilities::random(), 
                0.2, 
                b as f64 + 0.9 * utilities::random()
            );

            if (center - Point3f::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // Lambertian
                    let albedo: Vec3f = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::from_color(&albedo));
                    let center2: Point3f = center + Vec3f::new(0.0, utilities::random_f64_range(0.0, 0.5), 0.0);
                    scene.add(Arc::new(Sphere::new_moving(&center, &center2, 0.2, sphere_material)));
                } 
                else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = utilities::random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    scene.add(Arc::new(Sphere::new_stationary(&center, 0.2, sphere_material)));
                } 
                else {
                    // Dielectric
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    scene.add(Arc::new(Sphere::new_stationary(&center, 0.2, sphere_material)));
                }
            }
        }
    }

    let dielectric_material: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(0.0, 1.0, 0.0), 1.0, dielectric_material)));

    let lambertian_material: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(-4.0, 1.0, 0.0), 1.0, lambertian_material)));

    let metal_material: Arc<Metal> = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(4.0, 1.0, 0.0), 1.0, metal_material)));


    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(13.0, 2.0, 3.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.6;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn checkered_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let ground_texture : Arc<Checker> = Arc::new(
        Checker::from_color(
            0.32, 
            &Color::new(0.2, 0.3, 0.1),
            &Color::new(0.9, 0.9, 0.9),
        )
    );
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,-10.0,0.0), 
                10.0, 
                Arc::new(Lambertian::from_texture(ground_texture.clone()))
            )
        )
    );
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,10.0,0.0), 
                10.0, 
                Arc::new(Lambertian::from_texture(ground_texture.clone()))
            )
        )
    );

    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(13.0, 2.0, 3.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn earth() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let earth_filepath: &Path = Path::new("earthmap.png");
    let earth_texture : Arc<Image> = Arc::new(
        Image::read_image(earth_filepath).unwrap()
    );
    let earth_surface : Arc<Lambertian> = Arc::new(
        Lambertian::from_texture(earth_texture)
    );

    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,0.0,0.0), 
                2.0, 
                earth_surface
            )
        )
    );

    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(0.0, 0.0, 12.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn perlin_spheres() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let perlin_texture : Arc<Noise> = Arc::new(Noise::new(256, PerlinTexture::Marble(7), 4.0));
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,-1000.0,0.0), 
                1000.0, 
                Arc::new(Lambertian::from_texture(perlin_texture.clone()))
            )
        )
    );
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,2.0,0.0), 
                2.0, 
                Arc::new(Lambertian::from_texture(perlin_texture.clone()))
            )
        )
    );

    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(13.0, 2.0, 3.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn quads() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let left_red     = Arc::new(Lambertian::from_color(&Color::new(1.0, 0.2, 0.2)));
    let back_green   = Arc::new(Lambertian::from_color(&Color::new(0.2, 1.0, 0.2)));
    let right_blue   = Arc::new(Lambertian::from_color(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal   = Arc::new(Lambertian::from_color(&Color::new(0.2, 0.8, 0.8)));

    scene.add(Arc::new(Quad::new(
        &Point3f::new(-3.0,-2.0, 5.0),
        &Vec3f::new(0.0, 0.0, -4.0),
        &Vec3f::new(0.0, 4.0, 0.0),
        left_red)
    ));
    scene.add(Arc::new(Quad::new(
        &Point3f::new(-2.0,-2.0, 0.0),
        &Vec3f::new(4.0, 0.0, 0.0),
        &Vec3f::new(0.0, 4.0, 0.0),
        back_green)
    ));
    scene.add(Arc::new(Quad::new(
        &Point3f::new( 3.0,-2.0, 1.0),
        &Vec3f::new(0.0, 0.0, 4.0),
        &Vec3f::new(0.0, 4.0, 0.0),
        right_blue)
    ));
    scene.add(Arc::new(Quad::new(
        &Point3f::new(-2.0, 3.0, 1.0),
        &Vec3f::new(4.0, 0.0, 0.0),
        &Vec3f::new(0.0, 0.0, 4.0),
        upper_orange)
    ));
    scene.add(Arc::new(Quad::new(
        &Point3f::new(-2.0,-3.0, 5.0),
        &Vec3f::new(4.0, 0.0, 0.0),
        &Vec3f::new(0.0, 0.0, -4.0),
        lower_teal)
    ));

    // Camera
    let aspect_ratio: f64       = 1.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 80.0;
    let lookfrom: Point3f        = Point3f::new(0.0, 0.0, 9.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn simple_light() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let perlin_texture : Arc<Noise> = Arc::new(Noise::new(256, PerlinTexture::Marble(7), 4.0));
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,-1000.0,0.0), 
                1000.0, 
                Arc::new(Lambertian::from_texture(perlin_texture.clone()))
            )
        )
    );
    scene.add(
        Arc::new(
                Sphere::new_stationary(
                &Point3f::new(0.0,2.0,0.0), 
                2.0, 
                Arc::new(Lambertian::from_texture(perlin_texture.clone()))
            )
        )
    );

    let diffuse_light: Arc<DiffuseLight> = Arc::new(DiffuseLight::from_color(&Color::new(4.0, 4.0, 4.0)));
    scene.add(Arc::new(
        Sphere::new_stationary(
            &Point3f::new(0.0, 7.0, 0.0), 
            2.0,
            diffuse_light.clone()
        )
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(3.0, 1.0, -2.0), 
            &Vec3f::new(2.0, 0.0, 0.0), 
            &Vec3f::new(0.0, 2.0, 0.0), 
            diffuse_light.clone()
        )
    ));

    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.0, 0.0, 0.0);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(26.0, 3.0, 6.0);
    let lookat: Point3f          = Point3f::new(0.0, 2.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}

#[allow(dead_code)]
pub fn cornell_box() -> (HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let red: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: Arc<DiffuseLight> = Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0)));

    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(555.0, 0.0, 0.0),
            &Vec3f::new(0.0, 555.0, 0.0),
            &Vec3f::new(0.0, 0.0, 555.0),
            green.clone(),
        ),
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(0.0, 0.0, 0.0),
            &Vec3f::new(0.0, 555.0, 0.0),
            &Vec3f::new(0.0, 0.0, 555.0),
            red.clone(),
        ),
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(343.0, 554.0, 332.0),
            &Vec3f::new(-130.0, 0.0, 0.0),
            &Vec3f::new(0.0, 0.0, -105.0),
            light.clone(),
        ),
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(0.0, 0.0, 0.0),
            &Vec3f::new(555.0, 0.0, 0.0),
            &Vec3f::new(0.0, 0.0, 555.0),
            white.clone(),
        ),
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(555.0, 555.0, 555.0),
            &Vec3f::new(-555.0, 0.0, 0.0),
            &Vec3f::new(0.0, 0.0, -555.0),
            white.clone(),
        ),
    ));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(0.0, 0.0, 555.0),
            &Vec3f::new(555.0, 0.0, 0.0),
            &Vec3f::new(0.0, 555.0, 0.0),
            white.clone(),
        ),
    ));
    
    scene.add(Quad::new_box(
        &Point3f::new(130.0, 0.0, 65.0),
        &Point3f::new(295.0, 165.0, 230.0),
        white.clone())
    );
    scene.add(Quad::new_box(
        &Point3f::new(265.0, 0.0, 295.0),
        &Point3f::new(430.0, 330.0, 460.0),
        white.clone())
    );

    // Camera
    let aspect_ratio: f64       = 1.0;
    let image_width: u32        = 600;
    let samples_per_pixel: u32  = 200;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.0, 0.0, 0.0);

    let vertical_fov: f64       = 40.0;
    let lookfrom: Point3f        = Point3f::new(278.0, 278.0, -800.0);
    let lookat: Point3f          = Point3f::new(278.0, 278.0, 0.0);
    let vup: Vec3f               = Vec3f::new(0.0, 1.0, 0.0);

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, cam)
}