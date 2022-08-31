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