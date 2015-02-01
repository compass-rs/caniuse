extern crate "rustc-serialize" as rustc_serialize;


use rustc_serialize::json::{self};
use std::collections::HashMap;


mod agent;

#[derive(RustcDecodable)]
#[allow(dead_code)]
pub struct Link {
  url:String,
  title:String
}

#[derive(RustcDecodable)]
#[allow(dead_code)]
pub struct Feature {
  title: String,
  description: String,
  spec: String,
  status: String,
  links: Vec<Link>,
  categories: Vec<String>,
  stats: HashMap<String,HashMap<String,String>>,
  notes: String,
  notes_by_num: HashMap<String,String>,
  usage_perc_y: f32,
  usage_perc_a: f32,
  ucprefix: bool,
  parent: String,
  keywords: String
}

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct FromJson {
  agents:HashMap<String,agent::Agent>,
  statuses: HashMap<String,String>,
  cats: HashMap<String,Vec<String>>,
  updated: u64,
  data: HashMap<String,Feature>
}

pub struct Caniuse {
  from_json: FromJson
}

impl Caniuse {
  pub fn new() -> Caniuse {
    let raw = include_str!("caniuse.json");
    let from_json = json::decode::<FromJson>(raw).unwrap();
    let c = Caniuse {from_json:from_json};
    c
  }

  #[allow(dead_code)]
  pub fn has_data(&self) -> bool {
    self.from_json.agents.len() > 1
  }

  #[allow(dead_code)]
  pub fn agents(&self) -> &HashMap<String,agent::Agent> {
    &self.from_json.agents
  }

  #[allow(dead_code)]
  pub fn features(&self) -> &HashMap<String,Feature> {
    &self.from_json.data
  }

}


/*
# Returns all the known browsers according to caniuse
def browsers
@browsers ||= @data["agents"].keys.map{|b| PUBLIC_BROWSER_NAMES[b] }.sort
end
*/
