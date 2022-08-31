extern crate mech_core;
extern crate mech_utilities;
#[macro_use]
extern crate lazy_static;
use mech_core::*;
use mech_utilities::*;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
  static ref SEPARATOR: u64 = hash_string("separator");
  static ref STRING: u64 = hash_string("string");
  static ref TABLE: u64 = hash_string("table");
  static ref ROW: u64 = hash_string("row");
  static ref COLUMN: u64 = hash_string("column");
}


pub mod join;
pub mod length;
pub mod split;
