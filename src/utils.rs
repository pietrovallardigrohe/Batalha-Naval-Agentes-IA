use rand::{thread_rng, Rng};

pub fn random_point(mut limit_x: usize, mut limit_y: usize) -> [usize; 2] {
    let mut rng = thread_rng();
    limit_x += 1;
    limit_y += 1;
    let x = rng.gen_range(0..limit_x);
    let y = rng.gen_range(0..limit_y);

    [x, y]
}
