struct Process {
    pc: usize,
    regs: [i32; 16],
    carry: bool,
    wait_cycles: u32,
    player_id: i32,
}
new(player_id: i32, start_pc: usize) -> Process

fork(&self, new_pc: usize) -> Process // deep copy

advance_pc(&mut self, offset: usize) // wraps around arena