use crate::{interval::Interval, rand_f64, rand_f64_range, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl Color {
    pub fn rand() -> Color {
        return Color {
            x: rand_f64(),
            y: rand_f64(),
            z: rand_f64(),
        };
    }

    pub fn rand_range(min: f64, max: f64) -> Color {
        return Color {
            x: rand_f64_range(min, max),
            y: rand_f64_range(min, max),
            z: rand_f64_range(min, max),
        };
    }

    pub fn to_string(self) -> String {
        let intensity: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        let ir = (256.0 * intensity.clamp(r)) as i32;
        let ig = (256.0 * intensity.clamp(g)) as i32;
        let ib = (256.0 * intensity.clamp(b)) as i32;

        return format!("{ir} {ig} {ib}\n");
    }
}

#[macro_export]
macro_rules! write_color {
    ($out:expr, $color:expr) => {{
        let intensity: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let r = linear_to_gamma($color.x);
        let g = linear_to_gamma($color.y);
        let b = linear_to_gamma($color.z);

        let ir = (256.0 * intensity.clamp(r)) as i32;
        let ig = (256.0 * intensity.clamp(g)) as i32;
        let ib = (256.0 * intensity.clamp(b)) as i32;
        write!($out, "{ir} {ig} {ib}\n")?
    }};
}
