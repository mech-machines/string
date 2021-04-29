extern crate mech_core;
extern crate mech_utilities;
#[macro_use]
extern crate lazy_static;
use mech_core::{Transaction};
use mech_core::{Value, ValueMethods, IndexIterator, Table, TableIndex, ValueIterator};
use mech_core::{Quantity, ToQuantity, QuantityMath, make_quantity, hash_string};

lazy_static! {
  static ref SEPARATOR: u64 = hash_string("separator");
  static ref STRING: u64 = hash_string("string");
  static ref TABLE: u64 = hash_string("table");
  static ref ROW: u64 = hash_string("row");
  static ref COLUMN: u64 = hash_string("column");
}

#[no_mangle]
pub extern "C" fn string_split(arguments: &Vec<(u64, ValueIterator)>) {
  let (string_arg, string_vi) = &arguments[0];
  let (separator_arg, separator_vi) = &arguments[1];
  let (_, mut out) = arguments.last().unwrap().clone();
  if *string_arg == *TABLE && *separator_arg == *SEPARATOR {
    let separator = separator_vi.get_string(&TableIndex::Index(1), &TableIndex::Index(1)).unwrap();
    for row in 1..=string_vi.rows() {
      match string_vi.get_string(&TableIndex::Index(row), &TableIndex::Index(1)) {
        Some(string_value) => {
          let split_string = string_value.split(separator).collect::<Vec<_>>();
          out.resize(1,split_string.len());
          for (column, substring) in split_string.iter().enumerate() {
            out.set_string(&TableIndex::Index(row),&TableIndex::Index(column+1),Value::from_string(&substring.to_string()),substring.to_string());
          }
        }
        None => (),
      };
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn string_length(arguments: &Vec<(u64, ValueIterator)>) {
  let (string_arg, string_vi) = &arguments[0];
  let (_, mut out) = arguments.last().unwrap().clone();
  out.resize(string_vi.rows(),string_vi.columns());
  if *string_arg == *TABLE {
    for row in 1..=string_vi.rows() {
      for column in 1..=string_vi.columns() {
        match string_vi.get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some(string_value) => {
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
pub extern "C" fn string_join(arguments: &Vec<(u64, ValueIterator)>) {
  let (string_arg, string_vi) = &arguments[0];
  let (_, mut out) = arguments.last().unwrap().clone();
  out.resize(string_vi.rows(),string_vi.columns());
  if *string_arg == *COLUMN {
    out.resize(1,string_vi.columns());
    for column in 1..=string_vi.columns() {
      let mut output_string = "".to_string();
      for row in 1..=string_vi.rows() {
        match string_vi.get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some(string_value) => {
            output_string = format!("{}{}",output_string,string_value);
          }
          None => (),
        };
      }
      out.set_string(&TableIndex::Index(1),&TableIndex::Index(column),Value::from_string(&output_string.to_string()),output_string.to_string());
    }
  } else if *string_arg == *ROW {
    out.resize(string_vi.rows(),1);
    for row in 1..=string_vi.rows() {
      let mut output_string = "".to_string();
      for column in 1..=string_vi.columns() {
        match string_vi.get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some(string_value) => {
            output_string = format!("{}{}",output_string,string_value);
          }
          None => (),
        };
      }
      out.set_string(&TableIndex::Index(row),&TableIndex::Index(1),Value::from_string(&output_string.to_string()),output_string.to_string());
    }
  } else if *string_arg == *TABLE {
    let mut output_string = "".to_string();
    out.resize(1,1);
    for row in 1..=string_vi.rows() {
      for column in 1..=string_vi.columns() {
        match string_vi.get_string(&TableIndex::Index(row), &TableIndex::Index(column)) {
          Some(string_value) => {
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