use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material};
use crate::texture::ImageTexture;
use crate::triangle::Triangle;
use crate::vec3::*;
use crate::{bvh::BVHNode, material};
use std::sync::Arc;
use tobj::{load_obj, LoadOptions};

pub fn load_objects(
    pathname: &str,
    mat: Arc<dyn Material>,
    scale: f64,
    center: &mut Point3,
) -> HittableList {
    let pre_path = format!("objects/{}/", pathname);
    let obj_path = format!("{}{}.obj", pre_path, pathname);
    let (models, _) = load_obj(
        obj_path,
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
        let positions = &m.mesh.positions;
        let ind = &m.mesh.indices;
        let text_coordinates = &m.mesh.texcoords;
        let text_ind = &m.mesh.texcoord_indices;
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
        for i in (0..ind.len() - ind.len() % 3).step_by(3) {
            let mut uv = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
            if !text_coordinates.is_empty() {
                for j in 0..3 {
                    let index = text_ind[i + j] as usize;
                    uv[j] = (text_coordinates[2 * index], text_coordinates[2 * index + 1]);
                }
            }
            triangles.add(Arc::new(Triangle::new(
                &points[ind[i] as usize],
                &points[ind[i + 1] as usize],
                &points[ind[i + 2] as usize],
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
            )));
            tri = tri + 1;
        }
        // objects.add(Arc::new(triangles)); // TODO
        objects.add(Arc::new(BVHNode::new(&triangles, 0., 1.)));
    }
    println!("{}", tri);
    *center /= cnt as f64;
    if objects.objects.len() < 4 {
        return objects;
    }
    let mut list = HittableList::new();
    list.add(Arc::new(BVHNode::new(&objects, 0., 1.)));
    list
}

pub fn load_new(project_name: &str, scale: f64, col: &Color3, center: &mut Point3) -> HittableList {
    // .mtl

    let pre_path = format!("objects/{}/", project_name);
    let obj_path = format!("{}{}.obj", pre_path, project_name);
    let (models, materials) = load_obj(
        obj_path,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load .obj file.");

    // mtl
    let materials = materials.expect("Failed to load .mtl file.");
    let mut text: Vec<ImageTexture> = Vec::new();
    for mtl in materials {
        if let Some(texure_name) = mtl.diffuse_texture {
            let texure_path = format!("{}{}", pre_path, texure_name);
            let tex = ImageTexture::construct(&texure_path);
            text.push(tex);
        } else {
            text.push(ImageTexture::new());
        }
    }
    let mut objects = HittableList::new();
    let default_mat = Lambertian::construct(&col);
    let default_mat_ptr = Arc::new(default_mat);
    let mut cnt = 0;

    for m in models {
        let positions = &m.mesh.positions;
        let ind = &m.mesh.indices;
        let text_coordinates = &m.mesh.texcoords;
        let text_ind = &m.mesh.texcoord_indices;
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

        let cur_text = text[m.mesh.material_id.unwrap()].clone();
        let has_text = !text.is_empty();
        let mat = Lambertian::construct_texture(Arc::new(cur_text));
        let mat_ptr = Arc::new(mat);
        for i in 0..ind.len() / 3 {
            let mut uv = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
            if !text_coordinates.is_empty() {
                for j in 0..3 {
                    let index = text_ind[i * 3 + j] as usize;
                    uv[j] = (text_coordinates[index * 2], text_coordinates[index * 2 + 1]);
                }
            }
            if has_text {
                triangles.add(Arc::new(Triangle::new(
                    &points[ind[i * 3] as usize],
                    &points[ind[i * 3 + 1] as usize],
                    &points[ind[i * 3 + 2] as usize],
                    mat_ptr.clone(),
                    uv[0],
                    uv[1],
                    uv[2],
                )));
            } else {
                triangles.add(Arc::new(Triangle::new(
                    &points[ind[i * 3] as usize],
                    &points[ind[i * 3 + 1] as usize],
                    &points[ind[i * 3 + 2] as usize],
                    default_mat_ptr.clone(),
                    uv[0],
                    uv[1],
                    uv[2],
                )));
            }
        }
        objects.add(Arc::new(BVHNode::new(&triangles, 0.0, 1.0)));
    }
    *center /= cnt as f64;
    if objects.objects.len() < 4 {
        objects
    } else {
        let mut list = HittableList::new();
        list.add(Arc::new(BVHNode::new(&objects, 0., 1.)));
        list
    }
}
