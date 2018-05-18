fn main() {
    // zfs::snapshots::new("rpool/ROOT/ubuntu", "version2");
    let _ = zfs::snapshots::list();
    //zfs::snapshots::remove("rpool/ROOT/ubuntu@version2");
}

pub mod zfs {
    pub mod snapshots {
        use std::process::Command;

        pub fn new(pool_name: &str, snap_name: &str) {
            Command::new("zfs")
                .args(&["snapshot", &format!("{}@{}", pool_name, snap_name)])
                .output()
                .expect("failed to execute process");
        }

        pub fn list() -> Option<Vec<String>> {
            /*
            let output = Command::new("zfs")
                .args(&["list", "-t", "snapshots"])
                .output()
                .expect("failed to execute process");
            */

            let output = "NAME                         USED  AVAIL  REFER  MOUNTPOINT
                          rpool@gitlab                    0      -   128K  -
                          rpool/ROOT/ubuntu@version1  6,20M      -   643M  -
                          rpool/ROOT/ubuntu@gitlab1    211M      -   898M  -"
                .to_owned();
            let mut list: Vec<String> = Vec::new();
            for line in output.lines() {
                let name = line.split_whitespace().next();
                if let Some(snap_name) = name {
                    println!("{}", snap_name);
                    list.push(snap_name.to_owned());
                }
            }
            Some(
                list.into_iter()
                    .filter(|x| !(x == "NAME"))
                    .collect::<Vec<String>>(),
            )
        }

        pub fn remove(full_snap_name: &str) {
            Command::new("zfs")
                .args(&["destroy", full_snap_name])
                .output()
                .expect("failed to execute process");
        }
    }
}
