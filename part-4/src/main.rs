use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let x = Arc::new(Mutex::new(false));
    assignment(x.clone());
    let result = x.lock().unwrap();
    assert!(*result);
}

fn assignment(x: Arc<Mutex<bool>>) {
    todo!()
}

#[test]
fn mutate_shared_state() {
    let state = Arc::new(Mutex::new(false));
    assignment(state.clone());
    let result = state.lock().unwrap();
    assert!(*result);
}
