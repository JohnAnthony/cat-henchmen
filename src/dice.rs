use rand;
use rand::distributions::{IndependentSample, Range};

pub fn d(n: u64, sz: u64) -> u64 {
    let mut ret = 0;
    let mut rng = rand::thread_rng();
    let range = Range::new(1, sz+1);

    for _ in 0..n {
        ret += range.ind_sample(&mut rng);
    }

    ret
}
