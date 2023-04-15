use std::env;

// Pomo is a terminal based time tracking
//
// Features:
// - [ ] Start timing
// - [ ] End timing
// - [ ] Store time elapsed time
// - [ ] Integrate with Tmux


// TODO: fix if don´t pass any command this program broken

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1]; 

    if cmd == "start" {
        panic!("implement start");
    } else if cmd == "stop" {
        panic!("implement stop");
    }
}
