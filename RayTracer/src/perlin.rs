use crate::vec3::Vec3;
use crate::raytracer;

// static const int point_count = 256;
const point_count: i32 = 256;

pub struct perlin {
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
    // pub randfloat: Vec<f64>,
    pub randvec: Vec<Vec3>,
}
impl perlin {
    pub fn new() -> Self {
        // let mut randfloat = vec![0.0; point_count as usize];
        let mut randvec = vec![Vec3::zero(); point_count as usize];
        for i in 0..point_count {
            // randfloat[i as usize] = raytracer::random_double();
            randvec[i as usize] = Vec3::random_with_range(-1.0, 1.0).normalize();
        }
        Self {
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
            // randfloat,
            randvec,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        // let i = (4.0 * p.x) as i32 & 255;
        // let j = (4.0 * p.y) as i32 & 255;
        // let k = (4.0 * p.z) as i32 & 255;
        // return self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize];
        let mut u  = p.x - p.x.floor();
        let mut v  = p.y - p.y.floor();
        let mut w  = p.z - p.z.floor();
        // u = u * u * (3.0 - 2.0 * u);
        // v = v * v * (3.0 - 2.0 * v);
        // w = w * w * (3.0 - 2.0 * w);


        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        // let mut c = [[[0.0; 2]; 2]; 2];
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    // c[di as usize][dj as usize][dk as usize] = self.randfloat[(self.perm_x[((i+di) & 255) as usize] ^ self.perm_y[((j+dj) & 255) as usize] ^ self.perm_z[((k+dk) & 255) as usize]) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x[((i+di) & 255) as usize] ^ self.perm_y[((j+dj) & 255) as usize] ^ self.perm_z[((k+dk) & 255) as usize]) as usize];
                }
            }
        }
        // return Self::trilinear_interp(&c, u, v, w);
        return Self::perlin_interp(&c, u, v, w);   
    }
    // pub fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for i in 0..2 {
    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u)) *
    //                          (j as f64 * v + (1.0 - j as f64) * (1.0 - v)) *
    //                          (k as f64 * w + (1.0 - k as f64) * (1.0 - w)) * c[i as usize][j as usize][k as usize];
    //             }
    //         }
    //     }
    //     return accum;
    // }
    pub fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                             (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                             (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) * c[i as usize][j as usize][k as usize].dot(weight_v);
                }
            }
        }
        return accum;
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
    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += self.noise(&temp_p) * weight;
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        return accum.abs();
    }

}

