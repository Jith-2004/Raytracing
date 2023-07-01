use crate::vec3::Vec3;
use rand::Rng;

pub struct Perlin {
    ranvec: [Vec3; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    fn perlin_generate_perm() -> [usize; 256] {
        let mut rng = rand::thread_rng();
        let mut p: [usize; 256] = [0; 256];
        for i in 0..256 {
            p[i] = i;
        }
        for i in (1..=255).rev() {
            let target = rng.gen_range(0..i + 1) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
        p
    }

    fn trilinear_interp(c: [Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * Vec3::dot(c[i * 4 + j * 2 + k], weight_v);
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranvec: [Vec3; 256] = [Vec3::zero(); 256];
        for i in 0..256 {
            ranvec[i] = Vec3::unit_vector(Vec3::random_(-1.0, 1.0));
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x() - f64::floor(p.x()) as f64;
        let mut v = p.y() - f64::floor(p.y()) as f64;
        let mut w = p.z() - f64::floor(p.z()) as f64;
        let i = f64::floor(p.x()) as i32;
        let j = f64::floor(p.y()) as i32;
        let k = f64::floor(p.z()) as i32;
        let mut c: [Vec3; 8] = [Vec3::zero(); 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di * 4 + dj * 2 + dk] = self.ranvec[self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        return f64::abs(accum);
    }
}
