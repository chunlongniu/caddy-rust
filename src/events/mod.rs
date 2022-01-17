use std::collections::HashMap;

pub trait Handler {
  //  fn adapt(&self) -> Result<u8, u8>;
  //  fn add_package(&self) -> Result<u8, u8>;
  //  fn environ(&self) -> Result<u8, u8>;
  //  fn file_server(&self) -> Result<u8, u8>;
  //  fn fmt(&self) -> Result<u8, u8>;
  //  fn hash_password(&self) -> Result<u8, u8>;
  //  fn help(&self) -> Result<u8, u8>;
  //  fn list_modules(&self) -> Result<u8, u8>;
  //  fn reload(&self) -> Result<u8, u8>;
  //  fn remove_package(&self) -> Result<u8, u8>;
  //  fn reverse_package(&self) -> Result<u8, u8>;
  //  fn start(&self) -> Result<u8, u8>;
  //  fn run(&self) -> Result<u8, u8>;
    fn start(&self) -> Result<u8, u8>;
  //  fn stop(&self) -> Result<u8, u8>;
  //  fn trust(&self) -> Result<u8, u8>;
  //  fn untrust(&self) -> Result<u8, u8>;
  //  fn upgrade(&self) -> Result<u8, u8>;
  //  fn validate(&self) -> Result<u8, u8>;
  //  fn version(&self) -> Result<u8, u8>;
}
