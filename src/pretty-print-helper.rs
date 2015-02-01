/**
 * Use the following command line to get the expanded code for the Decodable trait,
 *
 *    rustc -Z unstable-options --pretty expanded src/marius.rs
 *
 * Then change "type_" to "type", format and save as agent.rs.
 * This is needed because the json uses "type".
 */

extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::Decodable;


#[derive(RustcDecodable)]
pub struct Agent {
  pub browser: String,
  pub prefix: String,
  pub abbr: String,
  type_: String,
  pub usage_global: HashMap<String,f32>
}
