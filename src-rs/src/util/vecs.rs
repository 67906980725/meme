pub fn index_of<T: std::cmp::PartialEq>(vec: &Vec<T>, one: &T) -> Option<isize> {
  for i in 0..vec.len() {
    if &vec[i] == one {
      return Some(i as isize);
    }
  }
  None
}

pub fn index_of_by<T: std::cmp::PartialEq, O>(vec: &Vec<O>, one: &T, f: fn(&O) -> T) -> Option<isize> {
  for i in 0..vec.len() {
    if &f(&vec[i]) == one {
      return Some(i as isize);
    }
  }
  None
}
