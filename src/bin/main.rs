use std::io;

mod ffi {
    #[link(name = "c")]
    extern "C" {
        pub fn epoll_create(size: i32) -> i32;
        pub fn close(fd: i32) -> i32;
        pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
        pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
    }

    #[repr(C)]
    pub struct Event {
        events: u32,
        epoll_data: usize,
    }
}

fn main() {
    // A counter to keep track of how many events we're expecting to act on
    let mut event_counter = 0;

    // First we create the event queue.
    // The size argument is ignored but needs to be larger than 0
    let queue = unsafe { ffi::epoll_create(1) };

    // This is how we basically check for errors and handle them using most
    // C APIs
    // We handle them by just panicking here in our example.
    if queue < 0 {
        panic!(io::Error::last_os_error());
    }
}
