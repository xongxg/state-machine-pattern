use std::time::Duration;

trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: Duration,
    shared_value: usize,
}

impl Waiting {
    fn new() -> Self {
        Waiting {
            waiting_time: Duration::new(0, 0),
            shared_value: 0,
        }
    }

    fn to_filling(&self) -> Filling {
        Filling {
            rate: 1,
            shared_value: 0,
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

struct Filling {
    shared_value: usize,
    rate: usize,
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[test]
fn test_shared_functionality() {
    let waiting = Waiting::new();
    let filling = waiting.to_filling();
}

struct Done;

enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
}

impl From<Waiting> for Filling {
    fn from(value: Waiting) -> Self {
        Filling {
            rate: 1,
            shared_value: value.get_shared_value(),
        }
    }
}

#[test]
fn test_states() {
    let waiting = State::Waiting(Waiting::new());
    // let filling = State::Filling(waiting.into());
    // println!("{:?}", filling.into());
}


