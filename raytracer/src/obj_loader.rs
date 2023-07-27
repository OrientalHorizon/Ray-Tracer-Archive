use crate::bvh::BVHNode;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::*;
use std::sync::Arc;
use tobj::{load_obj, LoadOptions};

pub fn load_objects(
    pathname: &str,
    mat: Arc<dyn Material>,
    scale: f64,
    center: &mut Point3,
) -> HittableList {
    let (models, _) = load_obj(
        pathname,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load .obj file.");
    let mut objects = HittableList::new();
    let mut tri = 0;
    *center = Point3::construct(&[0.0, 0.0, 0.0]);
    let mut cnt = 0;
    for m in models {
        let positions = &m.mesh.positions; //points position
        let indices = &m.mesh.indices; //points index (maybe joint)
        let mut points = Vec::new();
        let mut triangles = HittableList::new();

        for i in (0..positions.len()).step_by(3) {
            points.push(
                Point3::construct(&[positions[i], positions[i + 1], positions[i + 2]]) * scale,
            );
            cnt += 1;
            *center +=
                Point3::construct(&[positions[i], positions[i + 1], positions[i + 2]]) * scale;
        }
        for i in (0..indices.len() - indices.len() % 3).step_by(3) {
            triangles.add(Arc::new(Triangle::new(
                &points[indices[i] as usize],
                &points[indices[i + 1] as usize],
                &points[indices[i + 2] as usize],
                mat.clone(),
            )));
            tri = tri + 1;
        }
        // objects.add(Arc::new(triangles)); // TODO
        objects.add(Arc::new(BVHNode::new(&triangles, 0., 1.)));
    }
    println!("{}", tri);
    *center /= cnt as f64;
    // if objects.objects.len() < 4 {
    //     return objects;
    // }
    let mut list = HittableList::new();
    list.add(Arc::new(BVHNode::new(&objects, 0., 1.)));
    list
}
