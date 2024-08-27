use patch_core::Patch;
fn main() {
//    let mut patch = patch::Patch::from_name("add").unwrap();
 //   let data = vec![0x90, 0x90, 0x90];
  //  patch.write(data, 0xead);

     let mut patch = Patch::from_name("Finder").unwrap();
     let data = vec![0xb0, 0x01];
     patch.write(data, 0x8d0df);
}
