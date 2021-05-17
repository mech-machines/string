extern crate mech_core;
extern crate mech_utilities;
#[macro_use]
extern crate lazy_static;
use mech_core::{Transaction};
use mech_core::{Value, ValueMethods, IndexIterator, Table, TableIndex, ValueIterator};
use mech_core::{Quantity, ToQuantity, QuantityMath, hash_string, Argument};
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
  static ref SEPARATOR: u64 = hash_string("separator");
  static ref STRING: u64 = hash_string("string");
  static ref TABLE: u64 = hash_string("table");
  static ref ROW: u64 = hash_string("row");
  static ref COLUMN: u64 = hash_string("column");
}

#[no_mangle]
pub extern "C" fn string_split(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg1 = arguments[0].borrow();
  let string_arg = arg1.name;
  let string_vi = arg1.iterator.clone();
  let arg2 = arguments[0].borrow();
  let separator_arg = arg2.name;
  let separator_vi = arg2.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();
  
  if string_arg == *TABLE && separator_arg == *SEPARATOR {
    for row in 1..=string_vi.rows() {
      match (string_vi.table.borrow().get_string(&TableIndex::Index(row), &TableIndex::Index(1)),
             separator_vi.table.borrow().get_string(&TableIndex::Index(1), &TableIndex::Index(1))) {
        (Some((string_value,_)),Some((separator,_))) => {
          let split_string = string_value.split(separator).collect::<Vec<_>>();
          out.resize(1,split_string.len());
          for (column, substring) in split_string.iter().enumerate() {
            out.set_string(&TableIndex::Index(row),&TableIndex::Index(column+1),Value::from_string(&substring.to_string()),substring.to_string());
          }
        }
        _ => (),
      };
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn string_length(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg = arguments[0].borrow();
  let string_arg = arg.name;
  let string_vi = arg.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();

  out.resize(string_vi.rows(),string_vi.columns());
  if string_arg == *TABLE {
    for row in 1..=string_vi.rows() {
      for column in 1..=string_vi.columns() {
        match string_vi.table.borrow().get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some((string_value,_)) => {
            let length = string_value.len();
            out.set(&TableIndex::Index(row),&TableIndex::Index(column),Value::from_u64(length as u64));
          }
          None => (),
        };
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn string_join(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg = arguments[0].borrow();
  let string_arg = arg.name;
  let string_vi = arg.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();

  out.resize(string_vi.rows(),string_vi.columns());
  if string_arg == *COLUMN {
    out.resize(1,string_vi.columns());
    for column in 1..=string_vi.columns() {
      let mut output_string = "".to_string();
      for row in 1..=string_vi.rows() {
        match string_vi.table.borrow().get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some((string_value,_)) => {
            output_string = format!("{}{}",output_string,string_value);
          }
          None => (),
        };
      }
      out.set_string(&TableIndex::Index(1),&TableIndex::Index(column),Value::from_string(&output_string.to_string()),output_string.to_string());
    }
  } else if string_arg == *ROW {
    out.resize(string_vi.rows(),1);
    for row in 1..=string_vi.rows() {
      let mut output_string = "".to_string();
      for column in 1..=string_vi.columns() {
        match string_vi.table.borrow().get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some((string_value,_)) => {
            output_string = format!("{}{}",output_string,string_value);
          }
          None => (),
        };
      }
      out.set_string(&TableIndex::Index(row),&TableIndex::Index(1),Value::from_string(&output_string.to_string()),output_string.to_string());
    }
  } else if string_arg == *TABLE {
    let mut output_string = "".to_string();
    out.resize(1,1);
    for row in 1..=string_vi.rows() {
      for column in 1..=string_vi.columns() {
        match string_vi.table.borrow().get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some((string_value,_)) => {
            output_string = format!("{}{}",output_string,string_value);
          }
          None => (),
        };
      }
    }
    out.set_string(&TableIndex::Index(1),&TableIndex::Index(1),Value::from_string(&output_string.to_string()),output_string.to_string());
  } else {
    // TODO Warn about unknown argument
  }
}