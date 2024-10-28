use std::any::Any;

use sysinfo::{System,Disks};
fn main() {
    let sys = System::new_all();
    println!("cpu: {}", sys.cpus()[1].brand());
    println!("内存: {} GB", sys.total_memory() as f64 / 1024.0 / 1024.0/ 1024.0);
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("总计容量: {}GB 挂载点: {}, 类型: {:?}",disk.total_space() as f64 / 1024.0 / 1024.0/ 1024.0, disk.mount_point().to_string_lossy(), disk.name())
    }
}
