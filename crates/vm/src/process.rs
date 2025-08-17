struct Process {
    pub pc: usize,
    pub regs: [i32; config::REG_NUMBER],
    pub carry: bool,
    pub wait_cycles: u32,
    pub player_id: i32,
    pub last_live: u32,
    pub active: bool,
    pub name:String,
}
impl Process {
    pub fn new(player_id: i32, start_pc: usize,name:String) -> Process {
        Process {
            pc: start_pc,
            player_id,
            name
            regs: [0; config::REG_NUMBER],
            carry: false,
            wait_cycles: 0,
            active: true,
            last_live: 0,
        }
    }

    pub fn fork(&self, new_pc: usize) -> Process {
        Process {
            pc: new_pc,
            carry: self.carry,
            wait_cycles: 0,
            regs: self.regs,
            player_id: self.player_id,
            active: true,
            name:self.name;
            last_live: self.last_live,
        }
    }
    pub fn mark_live(&mut self, current_cycle: u32) {
        self.last_live = current_cycle;
        println!("Cycle {}: player{} ({}) is alive!",current_cycle,self.player_id,self.name);
    }
    pub fn step_pc(&mut self, offset: isize, mem_size: usize) {
        let new_pc = (self.pc as isize + offset).rem_euclid(mem_size as isize);
        self.pc = new_pc as usize;
    }
    fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
    pub fn read_reg(&self, idx: usize) -> i32 {
        self.regs[idx]
    }

    pub fn write_reg(&mut self, idx: usize, val: i32) {
        self.regs[idx] = val;
    }
}
