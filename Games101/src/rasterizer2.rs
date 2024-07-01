use std::collections::HashMap;
use std::time::Instant;

use nalgebra::{Matrix4, Vector3, Vector4, Vector2};
use crate::triangle::Triangle;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    // frame_sample: Vec<Vector3<f64>>,
    // depth_sample: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            // std::time::Instant::now()
            // let start = Instant::now();
            //self.rasterize_triangle(&t);
            //self.rasterize_triangle_msaa(&t);
            self.rasterize_triangle_fxaa(&t);
            // let end_time = start.elapsed();
            // println!("Time: {:?}", end_time);
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        //create AABB
        let mut min_x = t.v[0].x.min(t.v[1].x).min(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let min_x = min_x.floor() as usize;
        let mut max_x = t.v[0].x.max(t.v[1].x).max(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let max_x = max_x.ceil() as usize;
        let mut min_y = t.v[0].y.min(t.v[1].y).min(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let min_y = min_y.floor() as usize;
        let mut max_y = t.v[0].y.max(t.v[1].y).max(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let max_y = max_y.ceil() as usize;
        let v1 = Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z);
        let v2 = Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z);
        let v3 = Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z);
        //sample directly
        for x in min_x..max_x {
            for y in min_y..max_y {
                if inside_triangle(x as f64, y as f64, &[v1, v2, v3]) {
                    let (c1, c2, c3) = compute_barycentric2d(x as f64, y as f64, &[v1, v2, v3]);
                    let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                    let ind = self.get_index(x as usize, y as usize);
                    if z < self.depth_buf[ind] {
                        self.depth_buf[ind] = z;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &t.get_color());
                    }
                }
            }
        }  
    }
    pub fn rasterize_triangle_msaa(&mut self, t: &Triangle) {
        /*  implement your code here  */
        //create AABB
        let mut min_x = t.v[0].x.min(t.v[1].x).min(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let min_x = min_x.floor() as usize;
        let mut max_x = t.v[0].x.max(t.v[1].x).max(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let max_x = max_x.ceil() as usize;
        let mut min_y = t.v[0].y.min(t.v[1].y).min(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let min_y = min_y.floor() as usize;
        let mut max_y = t.v[0].y.max(t.v[1].y).max(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let max_y = max_y.ceil() as usize;
        let v1 = Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z);
        let v2 = Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z);
        let v3 = Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z);
        //sample with MSAA
        let sample_rate = 4;
        let sample_positions = [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
        for x in min_x..max_x {
            for y in min_y..max_y {
                let mut cnt = 0;
                let mut tot_depth = 0.0;
                let mut tot_color = Vector3::new(0.0, 0.0, 0.0);
                for(delta_x, delta_y) in sample_positions.iter() {
                    if inside_triangle(x as f64 + delta_x, y as f64 + delta_y, &[v1, v2, v3]) {
                        let (c1, c2, c3) = compute_barycentric2d(x as f64 + delta_x, y as f64 + delta_y, &[v1, v2, v3]);
                        let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                        let ind = self.get_index(x as usize, y as usize);
                        if z < self.depth_buf[ind] {
                            cnt += 1;
                            tot_depth += z;
                            tot_color += t.get_color();
                        }
                    }
                }
                if cnt > 0 {
                    let ind = self.get_index(x as usize, y as usize);
                    self.depth_buf[ind] = tot_depth / cnt as f64;
                    self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &(tot_color / sample_rate as f64));
                }
            }
        }
    }
    pub fn rasterize_triangle_fxaa(&mut self, t: &Triangle) {
        let mut min_x = t.v[0].x.min(t.v[1].x).min(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let min_x = min_x.floor() as usize;
        let mut max_x = t.v[0].x.max(t.v[1].x).max(t.v[2].x).max(0.0).min(self.width as f64 - 1.0);
        let max_x = max_x.ceil() as usize;
        let mut min_y = t.v[0].y.min(t.v[1].y).min(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let min_y =min_y.floor() as usize;
        let mut max_y = t.v[0].y.max(t.v[1].y).max(t.v[2].y).max(0.0).min(self.height as f64 - 1.0);
        let max_y = max_y.ceil() as usize;
        let v1 = Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z);
        let v2 = Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z);
        let v3 = Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z);
        for x in min_x..max_x {
            for y in min_y..max_y {
                if inside_triangle(x as f64, y as f64, &[v1, v2, v3]) {
                    let (c1, c2, c3) = compute_barycentric2d(x as f64, y as f64, &[v1, v2, v3]);
                    let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                    let ind = self.get_index(x as usize, y as usize);
                    if z < self.depth_buf[ind] {
                        self.depth_buf[ind] = z;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &t.get_color());
                    }
                }
            }
        } 

        //FXAA

        // let weight_red = 0.299;
        // let weight_green = 0.587;
        // let weight_blue = 0.114;
        let weight_red = 0.30;
        let weight_green = 0.59;
        let weight_blue = 0.11;
        let luma_gap = 0.15;

        for y in min_y as usize + 1..max_y as usize {
            for x in min_x as usize + 1..max_x as usize {
                let ind = self.get_index(x as usize, y as usize);
                let ind_right = self.get_index(x as usize + 1, y as usize);
                let ind_down = self.get_index(x as usize, y as usize + 1);
                let ind_up = self.get_index(x as usize, y as usize - 1);
                let ind_left = self.get_index(x as usize - 1, y as usize);

                let luma_origin = weight_red * self.frame_buf[ind].x + weight_green * self.frame_buf[ind].y + weight_blue * self.frame_buf[ind].z;
                let luma_right = weight_red * self.frame_buf[ind_right].x + weight_green * self.frame_buf[ind_right].y + weight_blue * self.frame_buf[ind_right].z;
                let luma_down = weight_red * self.frame_buf[ind_down].x + weight_green * self.frame_buf[ind_down].y + weight_blue * self.frame_buf[ind_down].z;
                let luma_up = weight_red * self.frame_buf[ind_up].x + weight_green * self.frame_buf[ind_up].y + weight_blue * self.frame_buf[ind_up].z;
                let luma_left = weight_red * self.frame_buf[ind_left].x + weight_green * self.frame_buf[ind_left].y + weight_blue * self.frame_buf[ind_left].z;

                let luma_max = luma_origin.max(luma_right).max(luma_down).max(luma_up).max(luma_left);
                let luma_min = luma_origin.min(luma_right).min(luma_down).min(luma_up).min(luma_left);

                let mut n = Vector2::new(0.0, 0.0);
                if luma_max - luma_min > luma_gap {
                    //let ver = fabs(luma_up - luma_origin) + fabs(luma_down - luma_origin);
                    let ver = (luma_up - luma_origin).abs() + (luma_down - luma_origin).abs();
                    // let ver = (luma_up + luma_down - 2.0 * luma_origin).abs();
                    // let hor = fabs(luma_left - luma_origin) + fabs(luma_right - luma_origin);
                    let hor = (luma_left - luma_origin).abs() + (luma_right - luma_origin).abs();
                    // let hor = (luma_left + luma_right - 2.0 * luma_origin).abs();
                    if ver > hor {
                        if (luma_up - luma_origin).abs() > (luma_down - luma_origin).abs() {
                            n.y = -1.0;
                        } else {
                            n.y = 1.0;
                        } 
                    } else {
                        if (luma_left - luma_origin).abs() > (luma_right - luma_origin).abs() {
                            n.x = -1.0;
                        } else {
                            n.x = 1.0;
                        }
                    }
                    let color = (self.frame_buf[ind] + self.frame_buf[self.get_index((x as f64 + n.x) as usize, (y as f64 + n.y) as usize)]) / 2.0;                             
                    self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &color);
                } 
            }
        }
        
    }



    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */
    let p = Vector2::new(x, y);
    let ed1 = Vector2::new(v[1].x - v[0].x, v[1].y - v[0].y);
    let ed2 = Vector2::new(v[2].x - v[1].x, v[2].y - v[1].y);
    let ed3 = Vector2::new(v[0].x - v[2].x, v[0].y - v[2].y);
    let vec1 = Vector2::new(p.x - v[0].x, p.y - v[0].y);
    let vec2 = Vector2::new(p.x - v[1].x, p.y - v[1].y);
    let vec3 = Vector2::new(p.x - v[2].x, p.y - v[2].y);
    let cross1 = ed1.x * vec1.y - ed1.y * vec1.x;
    let cross2 = ed2.x * vec2.y - ed2.y * vec2.x;
    let cross3 = ed3.x * vec3.y - ed3.y * vec3.x;
    (cross1 >= 0.0 && cross2 >= 0.0 && cross3 >= 0.0) || (cross1 <= 0.0 && cross2 <= 0.0 && cross3 <= 0.0)
}   

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}