pub mod ui;
pub mod watchpoint;

#[derive(Debug)]
pub struct Monitor {
    state: MonitorState,
}

#[derive(Debug, Clone)]
pub enum State {
    RUNNING,
    STOP,
    END,
    ABORT,
    QUIT,
}

#[derive(Clone, Debug)]
pub struct MonitorState {
    pub inner: State,
    pub halt_pc: usize,
    pub halt_ret: usize,
}

impl Monitor {
    pub fn start(self) {}

    pub fn exec(self) -> Result<i32, ()> {
        match self.state.inner {
            State::END | State::ABORT => {
                return Err(());
            }
            _ => {}
        }

        todo!()
    }
}
