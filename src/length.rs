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

