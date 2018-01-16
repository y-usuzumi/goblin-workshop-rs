use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Trait bound alias: GWId <==> Hash + Eq + Copy
pub trait GWId : Hash + Eq + Copy {}
impl<T: Hash + Eq + Copy> GWId for T {}

pub trait GWIdGen {
    type Id: Hash + Eq + Copy;
    fn gen(&self) -> Self::Id;
}

pub enum GWError {
    IdConflict,
    TaskNotExist
}

pub struct GWEnv;

type TaskFunc<TSuccess, TError> = fn(GWEnv) -> Result<TSuccess, TError>;

pub struct GW<TId: GWId, TSuccess, TError> {
    tasks: HashMap<TId, TaskFunc<TSuccess, TError>>,
    deps: HashMap<TId, TId>,
    rev_deps: HashMap<TId, TId>
}


impl<TId: GWId, TSuccess, TError> GW<TId, TSuccess, TError> {
    pub fn new() -> GW<TId, TSuccess, TError> {
        return Self {
            tasks: HashMap::new(),
            deps: HashMap::new(),
            rev_deps: HashMap::new()
        }
    }

    pub fn build_workshop(tasks: Vec<(TId, TaskFunc<TSuccess, TError>)>, deps: Vec<(TId, TId)>) -> GW<TId, TSuccess, TError> {
        let mut gw = Self::new();
        for (tid, func) in tasks {
            match gw.add_task(tid, func) {
                _ => {}
            }
        }
        for (first, then) in deps {
            match gw.set_after(first, then) {
                _ => {}
            }
        }
        return gw;
    }

    pub fn add_task(&mut self, id: TId, task: TaskFunc<TSuccess, TError>) -> Result<(), GWError> {
        match self.tasks.insert(id, task) {
            Some(_) => {
                return Err(GWError::IdConflict);
            },
            None => {
                return Ok(());
            }
        }
    }

    pub fn set_after(&mut self, first: TId, then: TId) -> Result<(), GWError> {
        if !self.tasks.contains_key(&first) ||
            !self.tasks.contains_key(&then) {
            return Err(GWError::TaskNotExist)
            }
        self.deps.insert(first, then);
        self.rev_deps.insert(then, first);
        return Ok(());
    }
}
