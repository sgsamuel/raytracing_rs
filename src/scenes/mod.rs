use std::path::Path;
use std::sync::Arc;

use crate::bvh_node::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Empty, Lambertian, Material, Metal};
use crate::perlin::PerlinTexture;
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{Checker, Image, Noise};
use crate::transform::{Translation, EulerRotation};
use crate::utilities;
use crate::vec3::{Point3f, Vec3f};

#[allow(dead_code)]
pub fn simple_spheres() -> (HittableList, HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let material_ground: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<Dielectric>   = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<Dielectric> = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Arc<Metal>       = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.2));

    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(0.0, -100.5,-1.0), 100.0, material_ground)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(0.0, 0.0,-1.2), 0.5, material_center)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(-1.0, 0.0,-1.0), 0.5, material_left)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(-1.0, 0.0,-1.0), 0.4, material_bubble)));
    scene.add(Arc::new(Sphere::new_stationary(&Point3f::new(1.0, 0.0,-1.0), 0.5, material_right)));


    // Camera
    let aspect_ratio: f64       = 16.0 / 9.0;
    let image_width: u32        = 400;
    let samples_per_pixel: u32  = 100;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.70, 0.80, 1.00);

    let vertical_fov: f64       = 20.0;
    let lookfrom: Point3f        = Point3f::new(-2.0, 2.0, 1.0);
    let lookat: Point3f          = Point3f::new(0.0, 0.0, -1.0);
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 10.0;
    let focus_dist: f64         = 3.4;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn bouncing_spheres() -> (HittableList, HittableList, Camera) {
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
                    let albedo: Color = Color::random_in_range(0.5, 1.0);
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.6;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn checkered_spheres() -> (HittableList, HittableList, Camera) {
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn earth() -> (HittableList, HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let earth_filepath: &Path = Path::new("images/earthmap.png");
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn perlin_spheres() -> (HittableList, HittableList, Camera) {
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn quads() -> (HittableList, HittableList, Camera) {
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn simple_light() -> (HittableList, HittableList, Camera) {
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
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn cornell_box() -> (HittableList, HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let red: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));

    // Cornell box sides
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
    
    // Light
    let light: Arc<DiffuseLight> = Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0)));
    scene.add(Arc::new(
        Quad::new(
            &Point3f::new(343.0, 554.0, 332.0),
            &Vec3f::new(-130.0, 0.0, 0.0),
            &Vec3f::new(0.0, 0.0, -105.0),
            light.clone(),
        ),
    ));

    // Box
    let box1: Arc<HittableList> = Quad::new_box(
        &Point3f::new(0.0, 0.0, 0.0),
        &Point3f::new(165.0, 330.0, 165.0),
        white.clone()
    );
    let rotated_box1:Arc<EulerRotation>  = Arc::new(EulerRotation::new(
        box1,
        &Vec3f::new(0.0, 15.0, 0.0),
    ));
    let translated_box1:Arc<Translation>  = Arc::new(Translation::new(
        rotated_box1,
        &Point3f::new(265.0, 0.0, 295.0),
    ));
    scene.add(translated_box1);

    // Glass Sphere
    let glass: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    scene.add(Arc::new(
        Sphere::new_stationary(
            &Point3f::new(190.0, 90.0, 190.0),
            90.0, 
            glass)
    ));

    // Light Sources
    let empty_material: Arc<Empty> = Arc::new(Empty);
    let mut lights: HittableList = HittableList::new();
    lights.add(Arc::new(
    Quad::new(
        &Point3f::new(343.0, 554.0, 332.0), 
        &Vec3f::new(-130.0, 0.0, 0.0),
        &Vec3f::new(0.0, 0.0, -105.0),
        empty_material.clone()
        )
    ));
    lights.add(Arc::new(
        Sphere::new_stationary(
            &Point3f::new(190.0, 90.0, 190.0),
            90.0,
            empty_material.clone()
        )
    ));

    // Camera
    let aspect_ratio: f64       = 1.0;
    let image_width: u32        = 600;
    let samples_per_pixel: u32  = 1000;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.0, 0.0, 0.0);

    let vertical_fov: f64       = 40.0;
    let lookfrom: Point3f        = Point3f::new(278.0, 278.0, -800.0);
    let lookat: Point3f          = Point3f::new(278.0, 278.0, 0.0);
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, lights, cam)
}

#[allow(dead_code)]
pub fn cornell_smoke() -> (HittableList, HittableList, Camera) {
    // Scene
    let mut scene: HittableList = HittableList::new();

    let red: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: Arc<DiffuseLight> = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));

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
            &Point3f::new(113.0, 554.0, 127.0),
            &Vec3f::new(330.0, 0.0, 0.0),
            &Vec3f::new(0.0, 0.0, 305.0),
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
    
    let box1: Arc<HittableList> = Quad::new_box(
        &Point3f::new(0.0, 0.0, 0.0),
        &Point3f::new(165.0, 330.0, 165.0),
        white.clone()
    );
    let rotated_box1:Arc<EulerRotation>  = Arc::new(EulerRotation::new(
        box1,
        &Vec3f::new(0.0, 15.0, 0.0),
    ));
    let translated_box1:Arc<Translation>  = Arc::new(Translation::new(
        rotated_box1,
        &Point3f::new(265.0, 0.0, 295.0),
    ));
    scene.add(Arc::new(ConstantMedium::from_color(translated_box1, 0.01, &Color::ZERO)));

    let box2: Arc<HittableList> = Quad::new_box(
        &Point3f::new(0.0, 0.0, 0.0),
        &Point3f::new(165.0, 165.0, 165.0),
        white.clone()
    );
    let translated_box2:Arc<Translation> = Arc::new(Translation::new(
        box2,
        &Point3f::new(130.0, 0.0, 65.0),
    ));
    let rotated_box2:Arc<EulerRotation>  = Arc::new(EulerRotation::new(
        translated_box2,
        &Vec3f::new(0.0, -18.0, 0.0),
    ));
    scene.add(Arc::new(ConstantMedium::from_color(rotated_box2, 0.01, &Color::ONE)));

    // Camera
    let aspect_ratio: f64       = 1.0;
    let image_width: u32        = 600;
    let samples_per_pixel: u32  = 200;
    let max_depth: u32          = 50;
    let background: Color       = Color::new(0.0, 0.0, 0.0);

    let vertical_fov: f64       = 40.0;
    let lookfrom: Point3f        = Point3f::new(278.0, 278.0, -800.0);
    let lookat: Point3f          = Point3f::new(278.0, 278.0, 0.0);
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}

#[allow(dead_code)]
pub fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) -> (HittableList, HittableList, Camera) {
    // World
    let mut scene: HittableList = HittableList::new();

    let mut boxes1: HittableList = HittableList::new();
    let ground: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side: u32 = 20;
    let w: f64 = 100.0;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = utilities::random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(
                Quad::new_box(
                    &Point3f::new(x0, y0, z0),
                    &Point3f::new(x1, y1, z1),
                    ground.clone(),
                )
            );
        }
    }

    scene.add(Arc::new(BVHNode::from_hittable_list(&mut boxes1)));

    let light: Arc<DiffuseLight> = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));
    scene.add(Arc::new(Quad::new(
        &Point3f::new(123.0, 554.0, 147.0),
        &Vec3f::new(300.0, 0.0, 0.0),
        &Vec3f::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1: Vec3f = Point3f::new(400.0, 400.0, 200.0);
    let center2: Vec3f = center1 + Vec3f::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.7, 0.3, 0.1)));
    scene.add(Arc::new(Sphere::new_moving(&center1, &center2, 50.0, sphere_material)));

    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary: Arc<Sphere> = Arc::new(Sphere::new_stationary(
        &Point3f::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.add(boundary.clone());
    scene.add(Arc::new(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));

    boundary = Arc::new(Sphere::new_stationary(
        &Point3f::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.add(Arc::new(ConstantMedium::from_color(
        boundary.clone(),
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let earth_filepath: &Path = Path::new("images/earthmap.png");
    let earth_texture : Arc<Image> = Arc::new(
        Image::read_image(earth_filepath).unwrap()
    );
    let earth_surface : Arc<Lambertian> = Arc::new(
        Lambertian::from_texture(earth_texture)
    );
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(400.0, 200.0, 400.0),
        100.0,
        earth_surface,
    )));

    let perlin_texture : Arc<Noise> = Arc::new(Noise::new(256, PerlinTexture::Marble(7), 0.2));
    scene.add(Arc::new(Sphere::new_stationary(
        &Point3f::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(perlin_texture)),
    )));

    let mut boxes2: HittableList = HittableList::new();
    let white: Arc<Lambertian> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let ns: u32 = 1000;

    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new_stationary(
            &Point3f::random_in_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    scene.add(Arc::new(Translation::new(
        Arc::new(EulerRotation::new(
            Arc::new(BVHNode::from_hittable_list(&mut boxes2)),
            &Vec3f::new(0.0, 15.0, 0.0),
        )),
        &Vec3f::new(-100.0, 270.0, 395.0),
    )));

    // Camera
    let aspect_ratio: f64       = 1.0;
    let image_width: u32        = image_width;
    let samples_per_pixel: u32  = samples_per_pixel;
    let max_depth: u32          = max_depth;
    let background: Color       = Color::new(0.0, 0.0, 0.0);

    let vertical_fov: f64       = 40.0;
    let lookfrom: Point3f        = Point3f::new(478.0, 278.0, -600.0);
    let lookat: Point3f          = Point3f::new(278.0, 278.0, 0.0);
    let vup: Vec3f               = Vec3f::E2;

    let defocus_angle: f64      = 0.0;
    let focus_dist: f64         = 10.0;

    let cam: Camera = Camera::new(
        aspect_ratio, image_width, samples_per_pixel, 
        max_depth, &background, vertical_fov, 
        &lookfrom, &lookat, &vup,
        defocus_angle, focus_dist
    );

    (scene, HittableList::new(), cam)
}