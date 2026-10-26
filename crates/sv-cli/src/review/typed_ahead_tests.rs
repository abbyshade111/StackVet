//! Backlog 0675: what was typed before a question was shown is dropped, on a real terminal. A pseudo-terminal stands
//! in for the owner's: a line is typed into it, and the terminal is asked whether anything waits to be read.

use super::typed_ahead::drop_pending;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};

/// A pseudo-terminal: the side a person types into, and the side `sv` reads.
fn terminal() -> (std::fs::File, std::fs::File) {
    // SAFETY: each call takes or returns a descriptor checked before it is used; `ptsname` returns a path the C library
    // owns, copied at once.
    unsafe {
        let typing = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        assert!(typing >= 0, "the setup: a pseudo-terminal");
        assert_eq!(libc::grantpt(typing), 0);
        assert_eq!(libc::unlockpt(typing), 0);
        let name = std::ffi::CStr::from_ptr(libc::ptsname(typing)).to_owned();
        let reading = libc::open(name.as_ptr(), libc::O_RDWR | libc::O_NOCTTY);
        assert!(reading >= 0, "the setup: its other side");
        (
            std::fs::File::from_raw_fd(typing),
            std::fs::File::from_raw_fd(reading),
        )
    }
}

/// Whether anything waits to be read, within a tenth of a second.
fn waiting(reading: &std::fs::File) -> bool {
    let mut poll = libc::pollfd {
        fd: reading.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    // SAFETY: one pollfd, for a descriptor this test holds open.
    unsafe { libc::poll(&mut poll, 1, 100) > 0 && poll.revents & libc::POLLIN != 0 }
}

#[test]
fn a_line_typed_before_the_question_is_dropped_and_the_next_is_read() {
    let (mut typing, mut reading) = terminal();
    typing.write_all(b"owner\n").unwrap();
    assert!(
        waiting(&reading),
        "the setup: the line typed ahead waits to be read"
    );
    drop_pending(reading.as_raw_fd());
    assert!(!waiting(&reading), "the line typed ahead is still there");
    // Typed after the question: read as the answer.
    typing.write_all(b"yes\n").unwrap();
    assert!(waiting(&reading));
    let mut read = [0u8; 16];
    let n = reading.read(&mut read).unwrap();
    assert_eq!(&read[..n], b"yes\n");
}

#[test]
fn on_something_that_is_not_a_terminal_nothing_is_dropped() {
    let mut pipe = [0; 2];
    // SAFETY: `pipe` fills two descriptors, owned by the files made from them here.
    assert_eq!(unsafe { libc::pipe(pipe.as_mut_ptr()) }, 0);
    let (mut reading, mut writing) = unsafe {
        (
            std::fs::File::from_raw_fd(pipe[0]),
            std::fs::File::from_raw_fd(pipe[1]),
        )
    };
    writing.write_all(b"owner\n").unwrap();
    drop_pending(reading.as_raw_fd());
    let mut read = [0u8; 16];
    let n = reading.read(&mut read).unwrap();
    assert_eq!(
        &read[..n],
        b"owner\n",
        "a test's answers come through a pipe, and are all read"
    );
}
