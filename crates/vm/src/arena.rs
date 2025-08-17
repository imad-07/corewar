pub struct Arena {
    mem: [u8; config::MEM_SIZE],
    main_proc: Vec<Process>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            mem: [0; config::MEM_SIZE],
            main_proc: vec![],
        }
    }
    pub fn get_proc(self, idx: u8) -> Option<Proc> {
        if idx >= self.main_proc.len() {
            return None;
        }
        self.main_proc[idx]
    }
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.mem[addr % config::MEM_SIZE]
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) {
        self.mem[addr % config::MEM_SIZE] = val;
    }
    pub fn read_i32(&self, addr: usize) -> i32 {
        let p1 = self.read_byte(addr);
        let p2 = self.read_byte(addr + 1);
        let p3 = self.read_byte(addr + 2);
        let p4 = self.read_byte(addr + 3);
        let res = (p1 as i32) << 8 | (p2 as i32) << 8 | (p3 as i32) << 8 | (p4 as i32) << 8;
        res
    }
    pub fn write_i32(&mut self, addr: usize, val: i32) {
        let bytes: [u8; 4] = val.to_be_bytes();
        self.write_byte(bytes[0]);
        self.write_byte(bytes[1]);
        self.write_byte(bytes[2]);
        self.write_byte(bytes[3]);
    }
    pub fn read_bytes(&self, addr: usize, n: usize) -> Vec<u8> {
        (0..n).map(|i| self.read_byte(addr + i)).collect()
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) {
        for (i, &b) in data.iter().enumerate() {
            self.write_byte(addr + i, b);
        }
    }
}
