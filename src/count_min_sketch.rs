use std::cmp::min;

pub struct CountMinSketch<const HASH_COUNT: usize> {
    counts: [[i32; 10]; HASH_COUNT]
}

impl CountMinSketch<2> {
    pub fn new() -> CountMinSketch<2> {
        CountMinSketch { counts: [[0; 10]; 2] }
    }

    pub fn incr(&mut self, key: &str) {
        self.counts[0][hash1(key) as usize] += 1;
        self.counts[1][hash2(key) as usize] += 1;
    }

    pub fn count(&self, key: &str) -> u32 {
        min(
            self.counts[0][hash1(key) as usize] as u32,
            self.counts[1][hash2(key) as usize] as u32,
        )
    }
}

fn hash1(input: &str) -> i32 {
    let mut res = 0;
    for byte in input.as_bytes() {
        res = res + byte;
    }

    (res % 10) as i32
}

fn hash2(input: &str) -> i32 {
    let mut res = 0;
    for byte in input.as_bytes() {
        res = (res*byte) % 100;
    }

    (res % 10) as i32
}

#[cfg(test)]
mod tests {
    use crate::count_min_sketch::CountMinSketch;

    #[test]
    fn it_works() {
        let mut sut = CountMinSketch::new();

        sut.incr("A");
        sut.incr("A");
        sut.incr("A");

        sut.incr("B");
        sut.incr("B");

        sut.incr("abcdefGHIjklMNOsadfhosahfdasdhfgh");
        sut.incr("abcdefGHIjklMNOsadfhosahfdasdhfgh");
        sut.incr("abcdefGHIjklMNOsadfhosahfdasdhfgh");
        sut.incr("abcdefGHIjklMNOsadfhosahfdasdhfgh");

        assert_eq!(sut.count("A"), 3);
        assert_eq!(sut.count("B"), 2);
        assert_eq!(sut.count("abcdefGHIjklMNOsadfhosahfdasdhfgh"), 4);
    }
}
