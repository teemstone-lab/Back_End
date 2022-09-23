use std::{thread::{self}, time::Duration};

use crate::pgmanager::dbmodel;

static mut THREAD_TERMINATE: bool = false;

pub fn sk_threadstart(dbinfo: &'static dbmodel::DBModel) {
  let _handle = thread::spawn(move || {
      loop{
          println!(" dbinfo.dbname == {}", dbinfo.dbname.to_string());
          println!(" dbinfo.host == {}", dbinfo.host.to_string());
          println!(" dbinfo.port == {}", dbinfo.port.to_string());
          println!(" dbinfo.user == {}", dbinfo.user.to_string());
          println!(" dbinfo.password == {}", dbinfo.password.to_string());
          thread::sleep(Duration::from_millis(5000));
          unsafe{
              if THREAD_TERMINATE == true {
                 break println!(" skThread Terminate...", );   
              }
          }
      }
  });
}

pub fn sk_threadstop() {
  unsafe{
    THREAD_TERMINATE = true;
    }
}
