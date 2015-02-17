
extern crate caniuse;


#[allow(dead_code)]
fn main() {
  let caniuse = caniuse::Caniuse::new();
  println!("caniuse agents");
  for (_,agent) in caniuse.agents().iter() {
    println!("  {} {}", agent.browser, agent.type_);
  }
  println!("caniuse features");
  for feature in caniuse.features().keys() {
    print!("  {}", feature);
  }
}
