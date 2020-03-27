#[derive(Debug)]
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
}
