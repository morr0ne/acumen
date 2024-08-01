use acumen::{Cpuinfo, OsRelease};

fn main() {
    let os = OsRelease::new();
    let cpuinfo = Cpuinfo::new();

    dbg!(os);
    dbg!(cpuinfo);
}
