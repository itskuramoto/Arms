use nix::sys::signal::{SaFlags, Signal};
use nix::sys::signal::{sigaction, SigAction, SigHandler, SigSet};

pub fn ignore_signals() {
    let sa = SigAction::new(
        SigHandler::SigIgn,
        SaFlags::empty(),
        SigSet::empty()
        );
    unsafe {
        sigaction(Signal::SIGINT, &sa).unwrap();
        sigaction(Signal::SIGQUIT, &sa).unwrap();
    }
}

pub fn restore_signals() {
    let sa = SigAction::new(
        SigHandler::SigDfl,
        SaFlags::empty(),
        SigSet::empty()
        );
    unsafe {
        sigaction(Signal::SIGINT, &sa).unwrap();
        sigaction(Signal::SIGQUIT, &sa).unwrap();
    }
}
