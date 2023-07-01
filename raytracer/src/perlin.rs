use crate::vec3::Vec3;
use rand::Rng;

pub struct Perlin {
    ranfloat: [f64; 256],
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

    fn trilinear_interp(c: [f64; 8], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i * 4 + j * 2 + k];
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranfloat: [f64; 256] = [0.0; 256];
        for i in 0..256 {
            ranfloat[i] = rng.gen_range(0.0..1.0);
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x() - f64::floor(p.x()) as f64;
        let mut v = p.y() - f64::floor(p.y()) as f64;
        let mut w = p.z() - f64::floor(p.z()) as f64;
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let i = f64::floor(p.x()) as i32;
        let j = f64::floor(p.y()) as i32;
        let k = f64::floor(p.z()) as i32;
        let mut c: [f64; 8] = [0.0; 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di * 4 + dj * 2 + dk] = self.ranfloat[self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
}
