use std::sync::Arc;

use raytracing::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    rand_f64, rand_f64_range,
    ray::Point3,
    sphere::Sphere,
    vec3::Vec3,
};

fn main() -> std::io::Result<()> {
    // Create world without Mutex
    let mut world = HittableList::new();

    let ground_mat = Arc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat: ground_mat,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3 {
                x: a as f64 + 0.9 * rand_f64(),
                y: 0.2,
                z: b as f64 + 0.9 * rand_f64(),
            };

            if (center
                - Point3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .length()
                > 0.9
            {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    Arc::new(Lambertian {
                        albedo: Color::rand() * Color::rand(),
                    })
                } else if choose_mat < 0.95 {
                    Arc::new(Metal {
                        albedo: Color::rand_range(0.5, 1.0),
                        fuzz: rand_f64_range(0.0, 0.5),
                    })
                } else {
                    Arc::new(Dielectric {
                        refraction_index: 1.5,
                    })
                };

                world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    mat,
                }));
            }
        }
    }

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
        radius: 1.0,
        mat: Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
        radius: 0.85,
        mat: Arc::new(Dielectric {
            refraction_index: 1.0 / 1.5,
        }),
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: -2.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: Arc::new(Lambertian {
            albedo: Color {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        }),
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: Arc::new(Metal {
            albedo: Color {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        }),
    }));

    let look_from = Point3 {
        x: -8.0,
        y: 1.8,
        z: 12.0,
    };
    let look_at = Point3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    };

    let cam = Camera::new(
        16.0 / 9.0,
        1920,
        50,
        20.0,
        look_from,
        look_at,
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        (look_from
            - Point3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            })
        .length(),
    );

    // Wrap world in Arc before passing it
    let world = Arc::new(world);
    cam.render(world, "out.ppm")?;

    Ok(())
}
