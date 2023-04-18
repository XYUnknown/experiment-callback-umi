use std::time::{SystemTime};
use std::collections::BinaryHeap;
use serde::{Serialize, Deserialize};
use umi::proxy_lib::callback::CallBack;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    callback: CallBack,
    time: SystemTime
}

impl Entry {
    pub fn new(callback: CallBack, ready_at: SystemTime) -> Entry {
        let entry = Entry {
            time: ready_at,
            callback: callback,
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

/**
 * A server that stores events that will become ready at some time in the future.
 * Events can be sumbit to the server, and the server can send notifications when events 
 * become ready. By calling the method `run` on the server, the server will check if any
 * event is ready, and if so, it will send a notification.
 */
pub struct ReadyReminderServer {
    entries: BinaryHeap<Entry>
}

impl ReadyReminderServer {
    pub fn new() -> ReadyReminderServer {
        ReadyReminderServer {
            entries: BinaryHeap::new()
        }
    }

    pub fn submit_event(&mut self, callback: CallBack, ready_at: SystemTime) {
        let entry = Entry::new(callback, ready_at);
        (&mut self.entries).push(entry);
    }

    pub fn run(&mut self) {
        while (&self.entries).len() > 0 {
            let first = (&self.entries).peek();
            match first {
                Some(entry) => {
                    if entry.get_time() <= &SystemTime::now() {
                        let e = (&mut self.entries).pop();
                        let c = e.clone().unwrap().callback;
                        c.execute(); // execute the callback
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
        let e1 = Entry::new(CallBack::new("Hello World!".to_string()), SystemTime::now() + Duration::new(1, 0));
        let e2 = Entry::new(CallBack::new("Goodbye World!".to_string()), SystemTime::now() + Duration::new(3, 0));
        assert!(e1 > e2);
    }

    #[test]
    fn cmp_works() {
        let e1 = Entry::new(CallBack::new("Hello World!".to_string()), SystemTime::now() + Duration::new(1, 0));
        let e2 = Entry::new(CallBack::new("Goodbye World!".to_string()), SystemTime::now() + Duration::new(2, 0));
        assert_eq!(e1.cmp(&e2), std::cmp::Ordering::Greater);
    }
}
