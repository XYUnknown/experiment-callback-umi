use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use umi::endpoint::{UMIEndpoint, ResourceTable};
use umi::{register};
use umi::proxy_lib::callback::CallBack;

use applications::reminder::ready_reminder_callback_server_umi::{ReadyReminderServer};
use umi_macros_proc::{setup_packages, setup_registry, setup_proc_macros};
setup_packages!();
setup_registry!();
setup_proc_macros!();

fn main() {
    let mut table = RegistryTable::new();
    register!(table, ReadyReminderServerNew, ReadyReminderServer::new, fn() -> ReadyReminderServer, (ReadyReminderServer, ResultOp::Owned));
    register!(table, ReadyReminderServerSubmit, ReadyReminderServer::submit_event, fn(&mut ReadyReminderServer, CallBack, String, SystemTime), ((), ResultOp::Owned), ReadyReminderServer, CallBack, String, SystemTime, &mut ReadyReminderServer, CallBack, String, SystemTime);
    register!(table, ReadyReminderServerRun, ReadyReminderServer::run, fn(&mut ReadyReminderServer), ((), ResultOp::Owned), ReadyReminderServer, &mut ReadyReminderServer);

    let mut server = UMIEndpoint::new("127.0.0.1:3335");
    let vtable = Arc::new(Mutex::new(ResourceTable::new()));
    server.start(table, vtable);
}