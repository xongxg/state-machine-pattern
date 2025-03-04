use std::time::Duration;

enum State {
    Waiting { waiting_time: Duration },
    Filling { rate: usize },
    Done,
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Waiting {
                waiting_time: Duration::new(0, 0),
            },
        }
    }

    fn to_filling(&mut self) {
        self.state = match self.state {
            State::Waiting { waiting_time } => State::Filling { rate: 1 },
            _ => panic!("Invalid state transition!"),
        }
    }
}

#[test]
fn test_machine(){
    let mut machine = StateMachine::new();
    machine.to_filling();
}
