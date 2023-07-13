use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rt_weekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{
    dot, random_in_unit_sphere, random_unit_vector, reflect, refract, Color3, Point3, Vec3,
};
use std::ops::Deref;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        Color3::construct(&[0.0, 0.0, 0.0])
    }
}

pub struct Lambertian {
    // pub albedo: Color3,
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    // pub fn new() -> Self {
    //     Self {
    //         albedo: Color3::new(),
    //     }
    // }
    pub fn construct(a: &Color3) -> Self {
        Self {
            albedo: Rc::new(SolidColor::construct(a)),
        }
    }
    pub fn construct_texture(a: Rc<dyn Texture>) -> Self {
        Self {
            albedo: Rc::clone(&a),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::construct(&rec.p, &scatter_direction, r_in.time());
        *attenuation = self.albedo.deref().value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    // pub fn new() -> Self {
    //     Self {
    //         albedo: Color3::new(),
    //         fuzz: 0.0,
    //     }
    // }
    pub fn construct(albedo: &Color3, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = reflect(&r_in.direction().unit(), &rec.normal);
        *scattered = Ray::construct(
            &rec.p,
            &(reflected + self.fuzz * random_in_unit_sphere()),
            r_in.time(),
        );
        *attenuation = self.albedo;
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    // pub fn new() -> Self {
    //     Self { ir: 1.0 }
    // }
    pub fn construct(ir: f64) -> Self {
        Self { ir }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::construct(&[1.0, 1.0, 1.0]);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta: f64 = dot(&(-unit_direction), &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let direction: Vec3 =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        *scattered = Ray::construct(&rec.p, &direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}
impl DiffuseLight {
    // pub fn construct(emit: Rc<dyn Texture>) -> Self {
    //     Self {
    //         emit: Rc::clone(&emit),
    //     }
    // }
    pub fn construct_color(emit: &Color3) -> Self {
        Self {
            emit: Rc::new(SolidColor::construct(emit)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.deref().value(u, v, p)
    }
}
