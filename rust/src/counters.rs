//! A group of the processor's performance counters, counting this process's
//! work in user mode, as in cpp/counters.cpp. They are counted together, so
//! the group must be no bigger than the number of counters the processor has.
//! With no events, it counts nothing.

pub const MAX: usize = 6;

/// As the kernel returns a group: how many counters, then their values
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Counts {
    pub size: u64,
    pub value: [u64; MAX],
}

pub struct Counters {
    size: usize,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    leader: i32,
}

impl Counters {
    /// Each event is NAME=TYPE:CONFIG, with the type and config that
    /// perf_event_open takes: type 0 for the kernel's generic hardware events
    /// and 4 for the processor's own, and the config in hex
    #[cfg(target_os = "linux")]
    pub fn new(events: &[String]) -> Counters {
        use perf_event_open_sys as sys;
        let mut leader = -1;
        for event in events {
            let (_, code) = event.split_once('=').unwrap();
            let (kind, config) = code.split_once(':').unwrap();
            let mut attr = sys::bindings::perf_event_attr {
                size: size_of::<sys::bindings::perf_event_attr>() as u32,
                type_: kind.parse().unwrap(),
                config: u64::from_str_radix(config, 16).unwrap(),
                read_format: sys::bindings::PERF_FORMAT_GROUP as u64,
                ..Default::default()
            };
            attr.set_exclude_kernel(1);
            attr.set_exclude_hv(1);
            attr.set_disabled((leader == -1) as u64);
            let fd = unsafe { sys::perf_event_open(&mut attr, 0, -1, leader, 0) };
            if fd == -1 {
                panic!("{event}: {}", std::io::Error::last_os_error());
            }
            if leader == -1 {
                leader = fd;
            }
        }
        if leader != -1 {
            unsafe { sys::ioctls::ENABLE(leader, sys::bindings::PERF_IOC_FLAG_GROUP) };
        }
        Counters {
            size: events.len(),
            leader,
        }
    }

    /// Only Linux lets an unprivileged process count, and main.rs offers the
    /// option only there
    #[cfg(not(target_os = "linux"))]
    pub fn new(_events: &[String]) -> Counters {
        Counters { size: 0, leader: -1 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    #[cfg(target_os = "linux")]
    pub fn read(&self, counts: &mut Counts) {
        if self.leader != -1 {
            unsafe {
                libc::read(
                    self.leader,
                    counts as *mut Counts as *mut libc::c_void,
                    (1 + self.size) * size_of::<u64>(),
                )
            };
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read(&self, _counts: &mut Counts) {}
}
