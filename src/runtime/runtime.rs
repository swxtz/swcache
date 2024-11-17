pub struct RuntimeState {
    state: bool,
}

impl RuntimeState {
    pub fn new() -> Self {
        RuntimeState { state: true }
    }

    pub fn state(&self) -> bool {
        return self.state;
    }

    // pub fn set_runtime_state(&mut self, new_state: bool) {
    //     self.state = new_state
    // }
}
