use tobj::{load_obj, LoadOptions};

use crate::bvh::BVHNode;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub fn load_objects(
    file_name: &str,
    mat_ptr: Arc<dyn Material>,
    scale: f64,
    center: &mut Point3,
) -> HittableList {
    let (models, _) = load_obj(
        file_name,
        &LoadOptions {
            ignore_lines: true,
            ignore_points: true,
            ..LoadOptions::default()
        },
    )
    .expect("Failed .obj");
    let mut list = HittableList::new();
    *center = Point3::construct(&[0.0, 0.0, 0.0]);
    let mut cnt: f64 = 0.0;
    for m in models.iter() {
        let ind = &m.mesh.indices;
        // 三角形顶点下标
        let pos = &m.mesh.positions;
        // 表示点的数组
        let mut points: Vec<Vec3> = Vec::new();
        let mut triangles = HittableList::new();

        for i in 0..pos.len() / 3 {
            let mut p: Point3 = Point3::new();
            for j in 0..3 {
                p.e[j] = pos[(i * 3 + j) as usize] as f64 * scale;
            }
            points.push(p);
            *center += p;
            cnt = cnt + 1.0;
        }

        for i in 0..ind.len() / 3 {
            let mut v: Vec<Point3> = Vec::new();
            for j in 0..3 {
                v.push(points[ind[(i * 3 + j) as usize] as usize]);
            }
            triangles.add(Arc::new(Triangle::construct(&v, Arc::clone(&mat_ptr))));
        }
        list.add(Arc::new(BVHNode::construct2(&triangles, 0.0, 1.0)));
    }
    *center /= cnt;
    list
}
