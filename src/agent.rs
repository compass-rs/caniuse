use rustc_serialize::Decodable;
use std::collections::HashMap;

pub struct Agent {
  pub browser: String,
  pub prefix: String,
  pub abbr: String,
  pub type_: String,
  pub usage_global: HashMap<String,f32>
}

impl Decodable for Agent {
  fn decode<__D: ::rustc_serialize::Decoder>(decoder: &mut __D)
  -> ::std::result::Result<Agent, __D::Error> {
    decoder.read_struct("Agent", 6us,
    |_d| -> _
    ::std::result::Result::Ok(Agent{
      browser: match _d.read_struct_field("browser", 0us, Decodable::decode)
      {
        Ok(__try_var) => __try_var,
        Err(__try_var) => return Err(__try_var),
      },
      prefix: match _d.read_struct_field("prefix", 1us, Decodable::decode)
      {
        Ok(__try_var) => __try_var,
        Err(__try_var) => return Err(__try_var),
      },
      abbr: match _d.read_struct_field("abbr",2us,Decodable::decode)
      {
        Ok(__try_var) => __try_var,
        Err(__try_var) => return Err(__try_var),
      },
      type_: match _d.read_struct_field("type",3us,Decodable::decode)
      {
        Ok(__try_var) => __try_var,
        Err(__try_var) => return Err(__try_var),
      },
      usage_global: match _d.read_struct_field("usage_global", 4us, Decodable::decode)
      {
        Ok(__try_var) => __try_var,
        Err(__try_var) => return Err(__try_var),
      },
    }
    ))
  }
}
