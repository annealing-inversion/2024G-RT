use crate::vec3::Vec3;
use crate::raytracer;

// static const int point_count = 256;
const point_count: i32 = 256;

pub struct perlin {
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
    pub randfloat: Vec<f64>,
}
impl perlin {
    pub fn new() -> Self {
        let mut randfloat = vec![0.0; point_count as usize];
        for i in 0..point_count {
            randfloat[i as usize] = raytracer::random_double();
        }
        Self {
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
            randfloat,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;
        return self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize];
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; point_count as usize];
        for i in 0..point_count {
            p[i as usize] = i;
        }
        Self::permute(&mut p, point_count);
        return p;
    }
    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (0..n).rev() {
            // let target = raytracer::random_int(0, i);
            let target = raytracer::random_int_range(0, i as usize);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

}

