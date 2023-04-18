use std::time::{SystemTime};
use std::collections::BinaryHeap;
use serde::{Serialize, Deserialize};

use umi_macros_proc::{proxy_me, umi_init, umi_struct_method, setup_packages, setup_registry, setup_proc_macros};
use umi::proxy_lib::callback::{CallBack};
use umi::{remote};

setup_packages!();
setup_registry!();
setup_proc_macros!();

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    callback: CallBack,
    callback_addr: String,
    time: SystemTime
}

impl Entry {
    pub fn new(callback: CallBack, callback_addr: String, ready_at: SystemTime) -> Entry {
        let entry = Entry {
            time: ready_at,
            callback: callback,
            callback_addr: callback_addr
        };
        entry
    }

    pub fn get_time(&self) -> &SystemTime {
        &self.time
    }
}

// partial order by time
impl PartialOrd for Entry {
    /* this is manually changed for builing a min-heap with std BinaryHeap */
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.time.partial_cmp(&self.time)
    }

    fn lt(&self, other: &Self) -> bool {
        self.time > other.time
    }

    fn le(&self, other: &Self) -> bool {
        self.time >= other.time
    }

    fn gt(&self, other: &Self) -> bool {
        self.time < other.time
    }

    fn ge(&self, other: &Self) -> bool {
        self.time <= other.time
    }
}

// total order by time
impl Ord for Entry {
    /* this is manually changed for builing a min-heap with std BinaryHeap */
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[proxy_me]
pub struct ReadyReminderServer {
    entries: BinaryHeap<Entry>
}

impl ReadyReminderServer {
    #[umi_init]
    pub fn new() -> ReadyReminderServer {
        ReadyReminderServer {
            entries: BinaryHeap::new()
        }
    }

    #[umi_struct_method]
    pub fn submit_event(&mut self, callback: CallBack, callback_addr: String, ready_at: SystemTime) {
        let entry = Entry::new(callback, callback_addr, ready_at);
        (&mut self.entries).push(entry);
    }

    #[umi_struct_method]
    pub fn run(&mut self) {
        while (&self.entries).len() > 0 {
            let first = (&self.entries).peek();
            match first {
                Some(entry) => {
                    if entry.get_time() <= &SystemTime::now() {
                        let e = (&mut self.entries).pop();
                        let c = e.clone().unwrap().callback;
                        let c_addr = e.unwrap().callback_addr;
                        remote!(c_addr, c);
                    } else {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, Duration};

    #[test]
    fn gt_works() {
        let e1 = Entry::new(CallBack::new("Hello World!".to_string()), "localhost".to_string(), SystemTime::now() + Duration::new(1, 0));
        let e2 = Entry::new(CallBack::new("Goodbye World!".to_string()), "localhost".to_string(),SystemTime::now() + Duration::new(3, 0));
        assert!(e1 > e2);
    }

    #[test]
    fn cmp_works() {
        let e1 = Entry::new(CallBack::new("Hello World!".to_string()), "localhost".to_string(),SystemTime::now() + Duration::new(1, 0));
        let e2 = Entry::new(CallBack::new("Goodbye World!".to_string()), "localhost".to_string(),SystemTime::now() + Duration::new(2, 0));
        assert_eq!(e1.cmp(&e2), std::cmp::Ordering::Greater);
    }
}
