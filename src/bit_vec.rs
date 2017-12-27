use std::mem;

#[derive(Debug)]
pub struct BitVec {
    bits: Vec<usize>,
    one_bits: usize,
}
impl BitVec {
    pub fn new(number_of_bits_hint: usize) -> Self {
        let size = (number_of_bits_hint + usize_bitwidth() - 1) / usize_bitwidth();
        BitVec {
            bits: vec![0; size],
            one_bits: 0,
        }
    }

    #[inline]
    pub fn insert(&mut self, index: usize) {
        let base = index / usize_bitwidth();
        let offset = index % usize_bitwidth();
        let block = self.bits.get_mut(base).expect("Never fails");
        if (*block & (1 << offset)) == 0 {
            *block |= 1 << offset;
            self.one_bits += 1;
        }
    }

    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        let base = index / usize_bitwidth();
        let offset = index % usize_bitwidth();
        (self.bits[base] & (1 << offset)) != 0
    }

    pub fn number_of_bits(&self) -> usize {
        self.bits.len() * usize_bitwidth()
    }

    pub fn number_of_one_bits(&self) -> usize {
        self.one_bits
    }
}

#[inline]
fn usize_bitwidth() -> usize {
    mem::size_of::<usize>() * 8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let bits = BitVec::new(0);
        assert_eq!(bits.number_of_bits(), 0);
        assert_eq!(bits.number_of_one_bits(), 0);

        let bits = BitVec::new(10);
        assert_eq!(bits.number_of_bits(), 64);
        assert_eq!(bits.number_of_one_bits(), 0);

        let bits = BitVec::new(100);
        assert_eq!(bits.number_of_bits(), 128);
        assert_eq!(bits.number_of_one_bits(), 0);
    }

    #[test]
    fn insert_and_contains_works() {
        let mut bits = BitVec::new(100);
        assert_eq!(bits.number_of_one_bits(), 0);
        assert_eq!(
            (0..bits.number_of_bits())
                .filter(|&i| bits.contains(i))
                .count(),
            0
        );

        bits.insert(30);
        assert!(bits.contains(30));
        assert_eq!(bits.number_of_one_bits(), 1);
        assert_eq!(
            (0..bits.number_of_bits())
                .filter(|&i| bits.contains(i))
                .count(),
            1
        );

        bits.insert(30);
        assert!(bits.contains(30));
        assert_eq!(bits.number_of_one_bits(), 1);

        for i in 0..bits.number_of_bits() {
            bits.insert(i);
        }
        assert_eq!(bits.number_of_one_bits(), bits.number_of_bits());
        assert_eq!(
            (0..bits.number_of_bits())
                .filter(|&i| bits.contains(i))
                .count(),
            bits.number_of_bits()
        );
    }
}
