use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use umi::endpoint::{UMIEndpoint, ResourceTable};
use umi::proxy_lib::callback::{CallBack};
use umi::{remote};

use umi_macros_proc::{setup_packages, setup_registry, setup_proc_macros};
use applications::reminder::ready_reminder_callback_server_umi::{ReadyReminderServer};

setup_packages!();
setup_registry!();
setup_proc_macros!();

fn main() {
    let mut table = RegistryTable::new();
    let vtable = Arc::new(Mutex::new(ResourceTable::new()));
    let handle = thread::spawn(move || {
        let mut r = remote!("127.0.0.1:3335", ReadyReminderServer::new, ReadyReminderServer);
        r.submit_event(CallBack::new("Goodbye World!".to_string()), "127.0.0.1:3336".to_string(), SystemTime::now() + Duration::new(8, 0));
        r.submit_event(CallBack::new("Hello World!".to_string()), "127.0.0.1:3336".to_string(), SystemTime::now() + Duration::new(5, 0));
        r.run();
    });

    let mut listener = UMIEndpoint::new("127.0.0.1:3336");
    listener.start(table, vtable);

    handle.join().unwrap();
}