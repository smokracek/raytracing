use crate::{
    color::Color,
    deg_to_rad,
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    rand_f64,
    ray::{Point3, Ray},
    vec3::Vec3,
};

use rayon::prelude::*;
use std::{
    fs::OpenOptions,
    io::{stdout, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    cam_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let samples_per_pixel = 500;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let image_height = f64::max(image_width as f64 / aspect_ratio, 1.0) as i32;

        let center = look_from;

        // Determine viewport dimensions
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate u,v,w unit basis vectors for camera coordinate frame
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        // Vectors along horizontal and vertical edges of viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors between each pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate defocus disc basis vectors
        let defocus_radius = focus_dist * deg_to_rad(defocus_angle / 2.0).tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        return Camera {
            image_width,
            image_height,
            cam_center: center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disc_u,
            defocus_disc_v,
        };
    }

    pub fn render(&self, world: Arc<HittableList>, path: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        let w = self.image_width;
        let h = self.image_height;
        write!(file, "P3\n{w} {h}\n255\n")?;

        let progress = Arc::new(AtomicUsize::new(0)); // Progress counter

        let rows: Vec<String> = (0..h)
            .into_par_iter()
            .map(|j| {
                let mut row = String::new();
                for i in 0..w {
                    let mut pixel_color = Color {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color += Camera::ray_color(&r, self.max_depth, &world);
                    }
                    row.push_str(&format!(
                        "{}",
                        (self.pixel_samples_scale * pixel_color).to_string()
                    ));
                }

                // Update and display progress
                let completed = progress.fetch_add(1, Ordering::Relaxed) + 1;
                if completed % 10 == 0 || completed == h as usize {
                    // Reduce printing frequency
                    let percent = (completed as f64 / h as f64) * 100.0;
                    print!("\rRendering: {:.2}%", percent);
                    stdout().flush().unwrap();
                }

                row
            })
            .collect();

        // Write all rows sequentially to file
        for row in rows {
            write!(file, "{}", row)?;
        }

        print!("\rDone                      \n");

        Ok(())
    }

    /// Get a ray from camera defocus disc to target pixel at (`i`,`j`) plus some offset
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let orig = if self.defocus_angle <= 0.0 {
            self.cam_center
        } else {
            self.defocus_disc_sample()
        };
        let dir = pixel_sample - orig;

        Ray { orig, dir }
    }

    /// Vector to random point in \[-.5,-.5\] - \[+.5,+.5\] unit square.
    fn sample_square() -> Vec3 {
        Vec3 {
            x: rand_f64() - 0.5,
            y: rand_f64() - 0.5,
            z: 0.0,
        }
    }

    /// Returns a random point in the camera's defocus disc
    fn defocus_disc_sample(&self) -> Point3 {
        let p = Vec3::rand_in_unit_disc();
        self.cam_center + (p.x * self.defocus_disc_u) + (p.y * self.defocus_disc_v)
    }

    /// Calculate color for the ray based on what in the world it hits.
    /// Recursively bounces until `depth` is reached.
    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        match world.hit(
            r,
            &Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
        ) {
            Some(hit_record) => match hit_record.mat.scatter(r, &hit_record) {
                Some(scatter_result) => {
                    return scatter_result.attenuation
                        * Self::ray_color(&scatter_result.scattered, depth - 1, world);
                }
                None => {
                    return Color {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    }
                }
            },
            None => {}
        }

        let unit_direction = Vec3::unit_vector(r.dir);
        let a = 0.5 * (unit_direction.y + 1.0);

        return (1.0 - a)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + a * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
    }
}
