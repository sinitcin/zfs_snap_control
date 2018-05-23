extern crate chrono;

fn main() {
    zfs::snapshots::new("rpool/ROOT/ubuntu");
    let list = zfs::snapshots::list();
    println!("{:?}", list);
    zfs::snapshots::remove("rpool/ROOT/ubuntu@version2");
}

pub mod zfs {
    pub mod snapshots {
        use std::process::Command;
        use chrono::prelude::*; // https://github.com/chronotope/chrono

        pub fn new(pool_name: &str) {
            let dt = Local::now();
            let snap_name = format!("{}_{}_{}__{}_{}", dt.day(), dt.month(), dt.year(), dt.hour(), dt.minute());
            let buffer = Command::new("zfs")
                .args(&["snapshot", &format!("{}@{}", pool_name, snap_name)])
                .output()
                .expect("Не смог создать снапшот");
        }

        pub fn list() -> Vec<String> {
            /*
            let output = Command::new("zfs")
                .args(&["list", "-t", "snapshots"])
                .output()
                .expect("Не могу получить список снапшотов");
            */

            let output = "NAME                         USED  AVAIL  REFER  MOUNTPOINT
                          rpool@gitlab                    0      -   128K  -
                          rpool/ROOT/ubuntu@version1  6,20M      -   643M  -
                          rpool/ROOT/ubuntu@gitlab1    211M      -   898M  -"
                .to_owned();

            output
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .filter(|s| s != &"NAME")
                .map(|s| s.to_owned())
                .collect()
        }

        pub fn remove(full_snap_name: &str) {
            Command::new("zfs")
                .args(&["destroy", full_snap_name])
                .output()
                .expect("Не могу уничтожить снапшот");
        }
    }
}
