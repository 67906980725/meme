use uuid::Uuid;

pub fn uuid_string() -> String {
  let uuid_obj = Uuid::new_v4();
  uuid_obj.to_string()
}
