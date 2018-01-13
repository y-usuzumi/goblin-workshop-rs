#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait RandGen {
    fn gen() -> Self;
}

pub struct WorkshopEnv;

type TaskFunc<T, E> = fn(WorkshopEnv) -> Result<T, E>;

pub struct Task<Id, T, E> {
    id: Id,
    func: TaskFunc<T, E>
}

impl<Id: RandGen, T, E> Task<Id, T, E> {
    pub fn new(func: TaskFunc<T, E>) -> Task<Id, T, E> {
        return Task {
            id: Id::gen(),
            func
        }
    }
}

impl<Id, T, E> Task<Id, T, E> {
    pub fn new_with_id(id: Id, func: TaskFunc<T, E>) -> Task<Id, T, E> {
        return Task {id, func}
    }
}

pub struct Workshop<Id, T, E> {
    tasks: Vec<Box<Task<Id, T, E>>>
}
