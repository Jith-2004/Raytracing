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
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}
