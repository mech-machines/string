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