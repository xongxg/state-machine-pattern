use std::time::Duration;

struct BottleFillingMachine<S> {
    state: S,
    shared_value: usize,
}

struct Waiting {
    waiting_time: Duration,
}

struct Filling {
    rate: usize,
}

struct Done;

impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            state: Waiting {
                waiting_time: Duration::new(0, 0),
            },
            shared_value,
        }
    }
}

impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(value: BottleFillingMachine<Waiting>) -> Self {
        BottleFillingMachine {
            shared_value: value.shared_value,
            state: Filling { rate: 1 },
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(value: BottleFillingMachine<Filling>) -> Self {
        BottleFillingMachine {
            state: Done,
            shared_value: value.shared_value,
        }
    }
}

impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(value: BottleFillingMachine<Done>) -> Self {
        BottleFillingMachine {
            state: Waiting {
                waiting_time: Duration::new(0, 0),
            },
            shared_value: value.shared_value,
        }
    }
}

#[test]
fn test_bottle_filling_machine() {
    let bottle_filler = BottleFillingMachine::new(0);
    // (Mock) Check on some shared and state-specific values
    assert_eq!(
        bottle_filler.state.waiting_time,
        std::time::Duration::new(0, 0)
    );
    assert_eq!(bottle_filler.shared_value, 0);

    let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);

    let bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
}

enum BottleFillingMachineWrapper {
    Waiting(BottleFillingMachine<Waiting>),
    Filling(BottleFillingMachine<Filling>),
    Done(BottleFillingMachine<Done>),
}

impl BottleFillingMachineWrapper {
    fn step(mut self) -> Self {
        self = match self {
            BottleFillingMachineWrapper::Waiting(w) => {
                BottleFillingMachineWrapper::Filling(w.into())
            }
            BottleFillingMachineWrapper::Filling(w) => BottleFillingMachineWrapper::Done(w.into()),
            BottleFillingMachineWrapper::Done(w) => BottleFillingMachineWrapper::Waiting(w.into()),
        };

        self
    }
}

struct Factory {
    bottle_filling_machine: BottleFillingMachineWrapper,
}

impl Factory {
    fn new() -> Self {
        Factory {
            bottle_filling_machine: BottleFillingMachineWrapper::Waiting(
                BottleFillingMachine::new(0),
            ),
        }
    }
}

#[test]
fn test_factory() {
    let mut the_factory = Factory::new();
    the_factory.bottle_filling_machine = the_factory.bottle_filling_machine.step();
    // println!("{}",the_factory.into());
}
