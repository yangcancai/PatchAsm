use patch_core::Patch;
fn main() {
    //    let mut patch = patch::Patch::from_name("add").unwrap();
    //   let data = vec![0x90, 0x90, 0x90];
    //  patch.write(data, 0xead);

    //   let mut patch = Patch::from_name("Finder").unwrap();
    //   let data = vec![0xb0, 0x01];
    //   patch.write(data, 0x8d0df);

    let mut p = Patch::from_name("add").unwrap();
    let base_addr = p.get_base_address().unwrap();
    let p1 = base_addr + 0x2008;
    // 你知道的十六进制地址
    let hex_address: u64 = p1 as u64; // 示例地址
    println!(
        "pid: {}, base_addr: {:x}, hex_address: {:x}",
        p.pid, base_addr, hex_address
    );
    let mut data = 0;
    let rs = p.read_u64(hex_address, &mut data);
    if rs {
        println!("read memory = 0x{:x}", data);
    }
}
