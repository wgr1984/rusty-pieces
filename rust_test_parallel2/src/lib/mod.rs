use std::fmt::*;

pub struct TaskTest {
  id: u32,
  info: String
}

impl TaskTest {

  pub fn new(id: u32) -> TaskTest {
    TaskTest {
      id,
      info: String::from("This is a test info!")
    }
  }

  pub fn print(&self) -> String {
    format!("id: {}, info {}", self.id, self.info)
  }
}

impl Display for TaskTest {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "id: {}, info {}", self.id, self.info)
  }
}
