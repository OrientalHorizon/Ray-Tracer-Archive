use crate::hittable::HitRecord;
use crate::pdf::{CosinePdf, Pdf};
use crate::ray::Ray;
use crate::rt_weekend::{random_double, PI};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{dot, random_in_unit_sphere, reflect, refract, Color3, Point3, Vec3};
// use std::ops::Deref;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        Color3::construct(&[0.0, 0.0, 0.0])
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Debug)]
pub struct Lambertian {
    // pub albedo: Color3,
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    // pub fn new() -> Self {
    //     Self {
    //         albedo: Color3::new(),
    //     }
    // }
    pub fn construct(a: &Color3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::construct(a)),
        }
    }
    pub fn construct_texture(a: Arc<dyn Texture>) -> Self {
        Self {
            albedo: Arc::clone(&a),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        // let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();

        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }
        // let direction = random_in_hemisphere(&rec.normal);
        // *scattered = Ray::construct(&rec.p, &direction.unit(), r_in.time());
        // *alb = self.albedo.value(rec.u, rec.v, &rec.p);
        // *pdf = 0.5 / PI;

        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::construct(&rec.normal)));
        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine: f64 = dot(&rec.normal, &scattered.direction().unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Debug)]
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected: Vec3 = reflect(&r_in.direction().unit(), &rec.normal);
        srec.specular_ray = Ray::construct(
            &rec.p,
            &(reflected + self.fuzz * random_in_unit_sphere()),
            0.0,
        );
        srec.attenuation = self.albedo;
        srec.is_specular = true;
        srec.pdf_ptr = None;
        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = None;
        srec.attenuation = Color3::construct(&[1.0, 1.0, 1.0]);
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

        srec.specular_ray = Ray::construct(&rec.p, &direction, r_in.time());
        true
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    // pub fn construct(emit: Arc<dyn Texture>) -> Self {
    //     Self {
    //         emit: Arc::clone(&emit),
    //     }
    // }
    pub fn construct_color(emit: &Color3) -> Self {
        Self {
            emit: Arc::new(SolidColor::construct(emit)),
        }
    }
}
impl Material for DiffuseLight {
    // fn scatter(
    //     &self,
    //     _r_in: &Ray,
    //     _rec: &HitRecord,
    //     _attenuation: &mut Vec3,
    //     _scattered: &mut Ray,
    // ) -> bool {
    //     false
    // }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color3 {
        if rec.front_face {
            self.emit.value(u, v, &p)
        } else {
            Vec3::construct(&[0.0, 0.0, 0.0])
        }
    }
}

// pub struct Isotropic {
//     albedo: Arc<dyn Texture>,
// }
// impl Isotropic {
//     // pub fn construct(albedo: Arc<dyn Texture>) -> Self {
//     //     Self {
//     //         albedo: Arc::clone(&albedo),
//     //     }
//     // }
//     pub fn construct_color(albedo: &Color3) -> Self {
//         Self {
//             albedo: Arc::new(SolidColor::construct(albedo)),
//         }
//     }
// }
// impl Material for Isotropic {
//     fn scatter(
//         &self,
//         r_in: &Ray,
//         rec: &HitRecord,
//         attenuation: &mut Vec3,
//         scattered: &mut Ray,
//     ) -> bool {
//         *scattered = Ray::construct(&rec.p, &random_in_unit_sphere(), r_in.time());
//         *attenuation = self.albedo.deref().value(rec.u, rec.v, &rec.p);
//         true
//     }
// }

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color3,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
}
impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            specular_ray: Ray::new(),
            is_specular: false,
            attenuation: Vec3::new(),
            pdf_ptr: None,
        }
    }
}
