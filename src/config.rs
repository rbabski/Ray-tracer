use crate::util::{camera::Camera, color::Color, hittable_list::HittableList, material::{Dielectric, Lambertian, Material, Metal}, vec3d::{Point3D, Vec3D}};


pub fn camera_init() -> Camera {
    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 100;

    camera.samples_per_pixel = 100;
    camera.max_depth = 5;

    camera.vfov = 90.0;
    camera.lookfrom = Point3D::new(-2.0, 2.0, 1.0); 
    camera.lookat = Point3D::new(0.0, 0.0, 0.0);
    camera.vup = Vec3D::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;


    camera
}


pub fn world_init() -> HittableList {
    let mut world = HittableList::default();

    //colors

    let gold = Color::new(0.8, 0.6, 0.2);

    // materials

    let lambertian_1 = Material::Lambertian(Lambertian { albedo: gold });
    let dielectric_1 = Material::Dielectric(Dielectric {refraction_index: 1.0/1.5});
    let metal_1  = Material::Metal(Metal { albedo: gold, fuzz: 0.1 });

    // world

    world
        .add_sphere(Point3D::new(0.0, 0.0, 0.0), 1.0, metal_1)
        .add_sphere(Point3D::new(0.0, 0.0, 0.0), 1.5, dielectric_1)
        .add_sphere(Point3D::new(0.0, 0.0, 1.0), 1.0, lambertian_1);


    world
}