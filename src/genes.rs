#[derive(Clone)]
pub struct Genes {
    inner: Vec<u8>
}

impl Genes {
    pub fn new(n: u32) -> Genes {
        // zero-initialized genes
        return Genes {
            inner: vec![0u8; (n / 8) as usize]
        };
    }

    pub fn new_with_genes(genes: Vec<u8>) -> Genes {
        return Genes {
            inner: genes
        };
    }

    /// get a single gene
    #[inline(always)]
    pub fn get(&self, idx: u32) -> u8 {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            return (self.inner[bucket] >> loc) & 1
        } else {
            return 0u8;
        }
    }

    /// set a single gene to 1
    #[inline(always)]
    pub fn set(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] |= 1 << loc;
        }
    }

    /// set a single gene to 0
    #[inline(always)]
    pub fn clear(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] &= !(1 << loc);
        }
    }

    #[inline(always)]
    pub fn flip(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] ^= 1 << loc;
        }
    }

    /// reset all the genes to 0
    #[inline(always)]
    pub fn wipe(&mut self) {
        for block in self.inner.iter_mut() {
            *block = 0;
        }
    }

    /// convenience method to get the nth 8-bit part of genes
    pub fn g8(&self, loc: usize) -> u8 {
        if loc < self.inner.len() {
            return self.inner[loc];
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 16-bit part of genes
    pub fn g16(&self, loc: usize) -> u16 {
        let adj_factor = 2;

        let loc = loc * adj_factor; // the Nth 16-bit slice is begins at 2N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u16 
                | ((self.inner[loc] as u16) << 8)
                | ((self.inner[loc + 1] as u16));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 32-bit part of genes
    pub fn g32(&self, loc: usize) -> u32 {
        let adj_factor = 4; 

        let loc = loc * adj_factor; // the Nth 32-bit slice is begins at 4N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u32
                | ((self.inner[loc] as u32) << 24)
                | ((self.inner[loc + 1] as u32) << 16)
                | ((self.inner[loc + 2] as u32) << 8)
                | ((self.inner[loc + 3] as u32));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 64-bit part of genes
    pub fn g64(&self, loc: usize) -> u64 {
        let adj_factor = 8; 

        let loc = loc * adj_factor; // the Nth 64-bit slice is begins at 8N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u64
                | ((self.inner[loc] as u64) << 56)
                | ((self.inner[loc + 1] as u64) << 48)
                | ((self.inner[loc + 2] as u64) << 40)
                | ((self.inner[loc + 3] as u64) << 32)
                | ((self.inner[loc + 4] as u64) << 24)
                | ((self.inner[loc + 5] as u64) << 16)
                | ((self.inner[loc + 6] as u64) << 8)
                | ((self.inner[loc + 7] as u64));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 128-bit part of genes
    pub fn g128(&self, loc: usize) -> u128 {
        let adj_factor = 16; 

        let loc = loc * adj_factor; // the Nth 128-bit slice is begins at 16N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u128
                | ((self.inner[loc] as u128) << 120)
                | ((self.inner[loc + 1] as u128) << 112)
                | ((self.inner[loc + 2] as u128) << 104)
                | ((self.inner[loc + 3] as u128) << 96)
                | ((self.inner[loc + 4] as u128) << 88)
                | ((self.inner[loc + 5] as u128) << 80)
                | ((self.inner[loc + 6] as u128) << 72)
                | ((self.inner[loc + 7] as u128) << 64)
                | ((self.inner[loc + 8] as u128) << 56)
                | ((self.inner[loc + 9] as u128) << 48)
                | ((self.inner[loc + 10] as u128) << 40)
                | ((self.inner[loc + 11] as u128) << 32)
                | ((self.inner[loc + 12] as u128) << 24)
                | ((self.inner[loc + 13] as u128) << 16)
                | ((self.inner[loc + 14] as u128) << 8)
                | ((self.inner[loc + 15] as u128));
        } else {
            return 0;
        }
    }
}