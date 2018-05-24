extern crate chrono;

fn main() {
    //zfs::snapshots::new("rpool/ROOT/ubuntu");
    let list = zfs::snapshots::list("rpool/ROOT/ubuntu");
    println!("{:?}", list);
    //zfs::snapshots::remove("rpool/ROOT/ubuntu@version2");
}

pub mod zfs {
    pub mod snapshots {
        extern crate chrono;
        use chrono::prelude::*;
        use std::process::Command; // https://github.com/chronotope/chrono

        pub fn new(pool_name: &str) {
            let dt = Utc::now();
            let snap_name = format!(
                "{}_{}_{}__{}_{}",
                dt.day(),
                dt.month(),
                dt.year(),
                dt.hour(),
                dt.minute()
            );
            let buffer = Command::new("zfs")
                .args(&["snapshot", &format!("{}@{}", pool_name, snap_name)])
                .output()
                .expect("Не смог создать снапшот");
            println!("{:?}", buffer);
        }

        pub fn list(pool_name: &str) -> Vec<chrono::DateTime<chrono::FixedOffset>> {
            /*
            let output = Command::new("zfs")
                .args(&["list", "-t", "snapshots"])
                .output()
                .expect("Не могу получить список снапшотов");
            */

            let output = "NAME                                 USED  AVAIL  REFER  MOUNTPOINT
                          rpool@gitlab                            0      -   128K  -
                          rpool/ROOT/ubuntu@version1          6,20M      -   643M  -
                          rpool/ROOT/ubuntu@gitlab1            235M      -   898M  -
                          rpool/ROOT/ubuntu@23_5_2018__17_50  95,9K      -  5,84G  -"
                .to_owned();

            // Получаем список снапшотов сделанных в pool_name именно нами
            output
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .filter(|line| line.find(pool_name) != None)
                .filter(|line| {
                    line.contains(|c: char| (c == '_' || c.is_ascii_digit()) && !c.is_ascii_digit())
                })
                .filter(|s| s != &"NAME")
                .filter_map(|line| line.rsplit('@').next())
                .map(|s| chrono::DateTime::parse_from_str(&format!("{} +00:00", s), "%e_%m_%Y__%H_%M %z").unwrap())
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
