use std::f64::INFINITY;

use indicatif::ProgressBar;

use crate::hittable;
use crate::hittable_list;
use crate::interval;
use crate::interval::Interval;
use crate::rays;
use crate::vectors;

pub struct Camera {
    // image
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: vectors::Point3,
    pixel00_loc: vectors::Point3,
    pixel_delta_u: vectors::Vec3,
    pixel_delta_v: vectors::Vec3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let center = vectors::Point3::new(0.0, 0.0, 0.0);

        let viewport_u = vectors::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vectors::Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center
            - vectors::Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &rays::Ray, world: &hittable_list::HittableList) -> vectors::Color {
        let hit_record = hittable::Hittable::hit(world, r, &interval::Interval::new(0.0, INFINITY));
        match hit_record {
            Some(rec) => 0.5 * (rec.normal + vectors::Color::new(1.0, 1.0, 1.0)),
            None => {
                let unit_direction = vectors::unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - a) * vectors::Color::new(1.0, 1.0, 1.0)
                    + vectors::Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    pub fn render<W: std::io::Write>(
        self,
        world: &hittable_list::HittableList,
        out: &mut W,
    ) -> Result<(), std::io::Error> {
        out.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;

        let bar = ProgressBar::new(self.image_height as u64);
        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let r = rays::Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                pixel_color.write_color(out)?;
            }
        }
        bar.finish();
        Ok(())
    }
}
