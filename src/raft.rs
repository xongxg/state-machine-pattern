/// https://hoverbear.org/blog/rust-state-machine-pattern/
/// raft demo by state machine

struct Leader {}

struct Candidate {}

struct Follower {}

struct Raft<S> {
    state: S,
}

impl Raft<Follower> {
    fn new() -> Self {
        Raft { state: Follower {} }
    }
}

impl From<Raft<Follower>> for Raft<Candidate> {
    fn from(value: Raft<Follower>) -> Self {
        Raft {
            state: Candidate {},
        }
    }
}

impl From<Raft<Candidate>> for Raft<Leader> {
    fn from(value: Raft<Candidate>) -> Self {
        Raft { state: Leader {} }
    }
}

impl From<Raft<Leader>> for Raft<Follower> {
    fn from(val: Raft<Leader>) -> Raft<Follower> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Follower { /* ... */ },
        }
    }
}

#[test]
fn test_raft() {
    let is_follower = Raft::new();
    let is_candidate = Raft::<Candidate>::from(is_follower);

    let is_leader = Raft::<Leader>::from(is_candidate);

    // Then it fails and rejoins later, becoming a Follower again.
    let is_follower_again = Raft::<Follower>::from(is_leader);

    // And goes up for election...
    let is_candidate_again = Raft::<Candidate>::from(is_follower_again);

    // But this time it fails!
    // let is_follower_another_time = Raft::<Follower>::from(is_candidate_again);
}
