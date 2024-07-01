#![allow(warnings)]
use std::os::raw::c_void;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};
use crate::shader::{FragmentShaderPayload, VertexShaderPayload};
use crate::texture::Texture;
use crate::triangle::Triangle;

pub type V3f = Vector3<f64>;
pub type M4f = Matrix4<f64>;

pub(crate) fn get_view_matrix(eye_pos: V3f) -> M4f {
    let mut view: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let t_view = Matrix4::new(1.0,0.0,0.0,-eye_pos.x,
                              0.0,1.0,0.0,-eye_pos.y,
                              0.0,0.0,1.0,-eye_pos.z,
                              0.0,0.0,0.0,1.0);
    let r_view = Matrix4::new(1.0,0.0,0.0,0.0,
                              0.0,1.0,0.0,0.0,
                              0.0,0.0,1.0,0.0,
                              0.0,0.0,0.0,1.0);
    view = r_view * t_view;
    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64,scale: f64,whether_to_use_another_axis: bool) -> M4f {
    let mut model: Matrix4<f64> = Matrix4::identity();
    if whether_to_use_another_axis == true {
        return model;
    }
    /*  implement your code here  */
    let mut scale = Matrix4::new(scale,0.0,0.0,0.0,
                                0.0,scale,0.0,0.0,
                                0.0,0.0,scale,0.0,
                                0.0,0.0,0.0,1.0);
    let ang = rotation_angle.to_radians();
    let mut rotation = Matrix4::new(ang.cos(),-ang.sin(),0.0,0.0,
                                    ang.sin(),ang.cos(),0.0,0.0,
                                    0.0,0.0,1.0,0.0,
                                    0.0,0.0,0.0,1.0);
    model = rotation * scale;
    model
}

pub(crate) fn get_model_matrix_lab3(rotation_angle: f64) -> M4f {
    let mut model: Matrix4<f64> = Matrix4::identity();
    let rad: f64 = rotation_angle.to_radians();
    model[(0, 0)] = rad.cos();
    model[(2, 2)] = model[(0, 0)];
    model[(0, 2)] = rad.sin();
    model[(2, 0)] = -model[(0, 2)];
    let mut scale: M4f = Matrix4::identity();
    scale[(0, 0)] = 2.5;
    scale[(1, 1)] = 2.5;
    scale[(2, 2)] = 2.5;
    model * scale 
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> M4f {
    let mut projection: Matrix4<f64> = Matrix4::identity();
    let mut scale: M4f = Matrix4::identity();
    /*  implement your code here  */
    let mut n = z_near;
    let mut f = z_far;
    let mut m_persp_to_ortho = Matrix4::new(n,0.0,0.0,0.0,
                                            0.0,n,0.0,0.0,
                                            0.0,0.0,n+f,-n*f,
                                            0.0,0.0,1.0,0.0);
    let mut t = -(eye_fov/2.0).to_radians().tan() * n.abs();
    let mut r = t * aspect_ratio;
    let mut l = -r;
    let mut b = -t;
    let mut m_ortho_1 = Matrix4::new(2.0/(r-l),0.0,0.0,0.0,
                                    0.0,2.0/(t-b),0.0,0.0,
                                    0.0,0.0,2.0/(n-f),0.0,
                                    0.0,0.0,0.0,1.0);
    let mut m_ortho_2 = Matrix4::new(1.0,0.0,0.0,-(r+l)/2.0,
                                    0.0,1.0,0.0,-(t+b)/2.0,
                                    0.0,0.0,1.0,-(n+f)/2.0,
                                    0.0,0.0,0.0,1.0);
    scale = m_ortho_1 * m_ortho_2;
    projection = scale * m_persp_to_ortho;
    projection
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<V3f>) -> Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}

pub fn load_triangles(obj_file: &str) -> Vec<Triangle> {
    let (models, _) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let n = mesh.indices.len() / 3;
    let mut triangles = vec![Triangle::default(); n];

    // 遍历模型的每个面
    for vtx in 0..n {
        let rg = vtx * 3..vtx * 3 + 3;
        let idx: Vec<_> = mesh.indices[rg.clone()].iter().map(|i| *i as usize).collect();

        // 记录图形每个面中连续三个顶点（小三角形）
        for j in 0..3 {
            let v = &mesh.positions[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_vertex(j, Vector4::new(v[0] as f64, v[1] as f64, v[2] as f64, 1.0));
            let ns = &mesh.normals[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_normal(j, Vector3::new(ns[0] as f64, ns[1] as f64, ns[2] as f64));
            let tex = &mesh.texcoords[2 * idx[j]..2 * idx[j] + 2];
            triangles[vtx].set_tex_coord(j, tex[0] as f64, tex[1] as f64);
        }
    }
    triangles
}

// 选择对应的Shader
pub fn choose_shader_texture(method: &str,
                             obj_path: &str) -> (fn(&FragmentShaderPayload) -> Vector3<f64>, Option<Texture>) {
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = phong_fragment_shader;
    let mut tex = None;
    if method == "normal" {
        println!("Rasterizing using the normal shader");
        active_shader = normal_fragment_shader;
    } else if method == "texture" {
        println!("Rasterizing using the normal shader");
        active_shader = texture_fragment_shader;
        tex = Some(Texture::new(&(obj_path.to_owned() + "spot_texture.png")));
    } else if method == "phong" {
        println!("Rasterizing using the phong shader");
        active_shader = phong_fragment_shader;
    } else if method == "bump" {
        println!("Rasterizing using the bump shader");
        active_shader = bump_fragment_shader;
    } else if method == "displacement" {
        println!("Rasterizing using the displacement shader");
        active_shader = displacement_fragment_shader;
    }
    (active_shader, tex)
}

pub fn vertex_shader(payload: &VertexShaderPayload) -> V3f {
    payload.position
}

#[derive(Default)]
struct Light {
    pub position: V3f,
    pub intensity: V3f,
}

pub fn normal_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let result_color =
        (payload.normal.xyz().normalize() + Vector3::new(1.0, 1.0, 1.0)) / 2.0;
    result_color * 255.0
}

pub fn phong_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    // 泛光、漫反射、高光系数
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    // 灯光位置和强度
    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    // ping point的信息
    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let mut result_color = Vector3::zeros(); // 保存光照结果
    
    // <遍历每一束光>
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let light_dir = (light.position - point).normalize();
        let view_dir = (eye_pos - point).normalize();
        let half_dir = (light_dir + view_dir).normalize();

        let r_squared = (light.position - point).norm_squared();

        let ambient = amb_light_intensity.component_mul(&ka);
        let diffuse = light.intensity.component_mul(&kd) / r_squared * f64::max(0.0, normal.normalize().dot(&light_dir));
        let specular = light.intensity.component_mul(&ks) / r_squared * f64::max(0.0, normal.normalize().dot(&half_dir)).powf(p);

        result_color += ambient + diffuse + specular;
    }
    result_color * 255.0
}

pub fn texture_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let texture_color: Vector3<f64> = match &payload.texture {
        // LAB3 TODO: Get the texture value at the texture coordinates of the current fragment
        // <获取材质颜色信息>

        None => Vector3::new(0.0, 0.0, 0.0),
        // Some(texture) => Vector3::new(0.0, 0.0, 0.0), // Do modification here
        Some(texture) => texture.get_color_bilinear(payload.tex_coords.x, payload.tex_coords.y),
    };
    let kd = texture_color / 255.0; // 材质颜色影响漫反射系数
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let color = texture_color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let mut result_color = Vector3::zeros();

    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let light_dir = (light.position - point).normalize();
        let view_dir = (eye_pos - point).normalize();
        let half_dir = (light_dir + view_dir).normalize();
        let r_squared = (light.position - point).norm_squared();
        let ambient = amb_light_intensity.component_mul(&ka);
        // let diffuse = light.intensity.component_mul(&kd) / r_squared * f64::max(0.0, normal.dot(&light_dir));
        // let specular = light.intensity.component_mul(&ks) / r_squared * f64::max(0.0, normal.dot(&half_dir)).powf(p);
        let diffuse = light.intensity.component_mul(&kd) / r_squared * f64::max(0.0, normal.normalize().dot(&light_dir));
        let specular = light.intensity.component_mul(&ks) / r_squared * f64::max(0.0, normal.normalize().dot(&half_dir)).powf(p);
        result_color += ambient + diffuse + specular;
    }

    result_color * 255.0
}

pub fn bump_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement bump mapping here 
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Normal n = normalize(TBN * ln)
    let n = normal;
    let (x, y, z) = (n.x, n.y, n.z);
    let t = Vector3::new(x * y / (x * x + z * z).sqrt(), (x * x + z * z).sqrt(), z * y / (x * x + z * z).sqrt());
    let b = n.cross(&t);
    let tbn = Matrix3::new(t.x, b.x, n.x, t.y, b.y, n.y, t.z, b.z, n.z);
    let u = payload.tex_coords.x;
    let v = payload.tex_coords.y;
    // let tex = payload.texture.unwrap();
    let tex = payload.texture.as_ref().unwrap();
    let w = tex.width as f64;
    let h = tex.height as f64;
    let dU = kh * kn * (tex.get_color_bilinear(u + 1.0 / w, v).norm() - tex.get_color_bilinear(u, v).norm());
    let dV = kh * kn * (tex.get_color_bilinear(u, v + 1.0 / h).norm() - tex.get_color_bilinear(u, v).norm());
    let ln = Vector3::new(-dU, -dV, 1.0);
    let normal = (tbn * ln).normalize();

    let mut result_color = Vector3::zeros();
    result_color = normal;

    result_color * 255.0
}

pub fn displacement_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let mut point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement displacement mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Position p = p + kn * n * h(u,v)
    // Normal n = normalize(TBN * ln)
    let n = normal;
    let (x, y, z) = (n.x, n.y, n.z);
    let t = Vector3::new(x * y / (x * x + z * z).sqrt(), (x * x + z * z).sqrt(), z * y / (x * x + z * z).sqrt());
    let b = n.cross(&t);
    let tbn = Matrix3::new(t.x, b.x, n.x, t.y, b.y, n.y, t.z, b.z, n.z);
    let u = payload.tex_coords.x;
    let v = payload.tex_coords.y;
    // let tex = payload.texture.unwrap();
    let tex = payload.texture.as_ref().unwrap();
    let w = tex.width as f64;
    let h = tex.height as f64;
    let dU = kh * kn * (tex.get_color_bilinear(u + 1.0 / w, v).norm() - tex.get_color_bilinear(u, v).norm());
    let dV = kh * kn * (tex.get_color_bilinear(u, v + 1.0 / h).norm() - tex.get_color_bilinear(u, v).norm());
    let ln = Vector3::new(-dU, -dV, 1.0);

    point = point + kn * normal.normalize() * tex.get_color_bilinear(u, v).norm();
    let n = (tbn * ln).normalize();

    let mut result_color = Vector3::zeros();
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let light_dir = (light.position - point).normalize();
        let view_dir = (eye_pos - point).normalize();
        let half_dir = (light_dir + view_dir).normalize();
        let r_squared = (light.position - point).norm_squared();
        let ambient = amb_light_intensity.component_mul(&ka);
        let diffuse = light.intensity.component_mul(&kd) / r_squared * f64::max(0.0, n.dot(&light_dir));
        let specular = light.intensity.component_mul(&ks) / r_squared * f64::max(0.0, n.dot(&half_dir)).powf(p);
        result_color += ambient + diffuse + specular;
    }

    result_color * 255.0
}
