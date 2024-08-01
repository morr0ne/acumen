use acumen::{Cpuinfo, Meminfo, OsRelease};

fn main() {
    let os = OsRelease::new();
    let cpuinfo = Cpuinfo::new();
    let meminfo = Meminfo::new();

    dbg!(os);
    dbg!(cpuinfo);
    dbg!(meminfo);
}
