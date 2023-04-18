use serde::{Serialize, Deserialize};
use umi_macros::{ToVariable, Variable};
use crate::proxy_lib::BorrowRemote;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CallBack {
    content: String
}

impl CallBack {
    pub fn new(content: String) -> CallBack { // time_to_due is in millisecondsfrom now
        CallBack {
            content: content,
        }
    }
    pub fn execute(&self) {
        println!("Task {} is due", self.content);
    }
}

impl ToVariable for CallBack {
    fn to_variable(self) -> Variable {
        let var = Variable::OwnedLocal(serde_json::to_string(&self).unwrap());
        var
    }
}

impl BorrowRemote for CallBack {
    fn borrow_remote(&self) -> Self {
        panic!("This should never be called");
    }
}