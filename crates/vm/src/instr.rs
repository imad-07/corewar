pub enum Operand{
Reg(usize),
Dir(usize),
Ind(usize)
}
fn instr_fork(proc: &mut Process, arena: &mut Arena, offset: isize) -> Process{

}
fn instr_lfork(proc: &mut Process, arena: &mut Arena, offset: isize) -> Process;

fn instr_sti(proc: &mut Process, arena: &mut Arena, src_reg: usize, addr1: Operand, addr2: Operand);

fn instr_add(proc: &mut Process, arena: &mut Arena, reg1: usize, reg2: usize, dst_reg: usize){
    
}
fn instr_sub(proc: &mut Process, arena: &mut Arena, reg1: usize, reg2: usize, dst_reg: usize);
fn instr_and(proc: &mut Process, arena: &mut Arena, op1: Operand, op2: Operand, dst_reg: usize);

fn instr_live(proc: &mut Process, arena: &mut Arena, value: i32, current_cycle: u32){
 if let p = arena.get_proc((-value) as usize) {
    p.mark_live(current_cycle);
   }
}
fn instr_zjmp(proc: &mut Process, arena: &mut Arena, offset: isize);

fn instr_ld(proc: &mut Process, arena: &mut Arena, src: Operand, dst_reg: usize);
fn instr_st(proc: &mut Process, arena: &mut Arena, src_reg: usize, dst: Operand){
    match dst_operand {
        Operand::Reg(dst_reg) => {
            let val = proc.read_reg(src_reg);
            proc.write_reg(dst_reg, val);
        }
        Operand::Indirect(offset) => {
            let addr = (proc.pc + (offset % IDX_MOD as i32)) as usize;
            let val = proc.read_reg(src_reg);
            arena.write_i32(addr, val);
        }
        _ => {/*not doing anything poor process*/},
    }
}

fn instr_nop(proc: &mut Process, arena: &mut Arena){}
