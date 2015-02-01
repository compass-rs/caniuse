
#[test]
fn test_caniuse_has_data() {
  let c = Caniuse::new();
  assert!( c.has_data() )
}
