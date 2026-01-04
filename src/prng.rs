pub struct Pcg32 {
    state : u64,
    inc : u64,
}

impl Default for Pcg32 {
    fn default() -> Self {
        Self {
            state: 0x853c49e6748fea9b_u64,
            inc: 0xda3e39cb94b95bdb_u64,
        }
    }
}

impl Pcg32 {
    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state.wrapping_mul(6364136223846793005).wrapping_add(self.inc); // so the overflow can be handled
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32; 
        let rotation_amount = (old_state >> 59 ) as u32;    
        // return (xorshifted >> rotation_amount) | (xorshifted << ((-rotation_amount) & 31)); rust doesnt handle negate on an uint
        xorshifted.rotate_right(rotation_amount)
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    } 

    pub fn new(init_state: u64, init_seq: u64) -> Self {
        let mut rng:Pcg32 = Pcg32 { state: (0), inc: ((init_seq << 1) | 1) };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(init_state);
        rng.next_f32();        

        rng
    }
}