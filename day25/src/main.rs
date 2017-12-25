
enum State {
    A,
    B,
    C,
    D,
    E,
    F
}

struct StateChange {
    writeValue: i32,
    moveValue: i32,
    nextState: State,
}

fn main() {
    let STOP_COUNT = 12425180;
    let mut values = vec![0; 4194304];
    let mut idx = 2097152;
    let mut totalSteps = 0;
    let mut currentState: State = State::A;

    while totalSteps < STOP_COUNT {
        match currentState {
            State::A => {
                if values[idx] == 0 {
                    values[idx] = 1;
                    idx = idx + 1;
                    currentState = State::B;
                } else {
                    values[idx] = 0;
                    idx = idx + 1;
                    currentState = State::F;
                }
            },

            State::B => {
                if values[idx] == 0 {
                    values[idx] = 0;
                    idx = idx - 1;
                    currentState = State::B;
                } else {
                    values[idx] = 1;
                    idx = idx - 1;
                    currentState = State::C;
                }
            },

            State::C => {
                if values[idx] == 0 {
                    values[idx] = 1;
                    idx = idx - 1;
                    currentState = State::D;
                } else {
                    values[idx] = 0;
                    idx = idx + 1;
                    currentState = State::C;
                }
            },

            State::D => {
                if values[idx] == 0 {
                    values[idx] = 1;
                    idx = idx - 1;
                    currentState = State::E;
                } else {
                    values[idx] = 1;
                    idx = idx + 1;
                    currentState = State::A;
                }
            },

            State::E => {
                if values[idx] == 0 {
                    values[idx] = 1;
                    idx = idx - 1;
                    currentState = State::F;
                } else {
                    values[idx] = 0;
                    idx = idx - 1;
                    currentState = State::D;
                }
            },

            State::F => {
                if values[idx] == 0 {
                    values[idx] = 1;
                    idx = idx + 1;
                    currentState = State::A;
                } else {
                    values[idx] = 0;
                    idx = idx - 1;
                    currentState = State::E;
                }
            },
        }
        totalSteps += 1;
    }

    let mut diagnostic = 0;
    for v in values {
        if v == 1 {
            diagnostic += 1;
        }
    }

    println!("Diag is {}", diagnostic);
}
