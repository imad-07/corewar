use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Process) {
        let nt = Process::new(self.track_worker(), c, self);
        self.states.borrow_mut().push(false);
        (nt.pid, nt)
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id){
            panic!("{id} is already dropped");
        }
        self.drops.replace(self.drops.get() + 1);
        self.states.replace_with(|o| {
            let mut n = vec![];
            for v in o {
                n.push(*v);
            }
            n[id] = true;
            n
        });
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Process<'a> {
    pub pid: usize,
    pub carry:bool,
    pub instructions: Vec<String>,
    pub ist_pos:usize,
    pub parent: &'a Workers,
}

impl<'a> Process<'a> {
    pub fn new(p: usize, c: Vec<String>,carry:bool t: &'a Workers) -> Process {
        Process {
            pid: p,
            instructions: c,
            carry:carry,
            ist_pos:0,
            parent: t,
        }
    }
    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
    pub fn fork(self)->Process{
      let id = self.parent.track_worker();
      Process {
        pid: id,
        instructions: self.instructions,
        carry:self.carry,
        ist_pos:self.ist_pos,
        parent: self.parent,
    }
    }
}