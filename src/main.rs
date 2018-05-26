extern crate chrono;
extern crate clap;
use chrono::prelude::*;
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("     
-------------------------------------------------------------------------------                                                                
   mmm   m   m  m   m  m   m  m   m  m    m m   m         mmmm   m   m   mmm  
  #\"  \"  #  ##  #   #  #  ##  #   #  #    # #   #         #\" \"#  \"m m\"  #\"  \" 
  #      # # #  #\"\"\"#  # # #  #   #  #\"\"m # #\"\"\"#         #   #   #m#   #     
  \"#mm\"  #\"  #  #   #  #\"  #  #mmm#m #mm\" # #   #    #    ##m#\"   \"#    \"#mm\" 
                                   #                      #       m\"          
                                                          \"      \"\"           
------------------------------ ZFS Snap Control -------------------------------
    ")
        .about("Create ZFS snapshots on a schedule and automatically delete old ones!")
        .version("Version: 1.0")
        .author("     Anton Sinicin (c) 2018")
        .arg(
            Arg::with_name("pool_name")
                .short("p")
                .long("pool_name")
                .value_name("FILE")
                .help("Sets a custom pool for are control of snapshots.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("days_duration")
                .short("d")
                .long("days duration")
                .value_name("COUNT")
                .help("Set the length of days during which the state snapshots will be taken.")
                .takes_value(true),
        )
        .get_matches();

    let pool_name;
    match matches.value_of("pool_name") {
        Some(foo) => pool_name = foo,
        None => {
            println!(
                "[ERROR] => Without arg \"pool_name\", it's impossible to continue working..."
            );
            process::exit(0);
        }
    }
    let days_duration = matches
        .value_of("days_duration")
        .unwrap_or("14")
        .parse()
        .unwrap_or(14);

    // Удаляем старые снапшоты
    let dt = Utc::now();
    let list = zfs::snapshots::list(pool_name);
    for item in list {
        let fixed_dt = dt.with_timezone(&FixedOffset::east(0));
        println!("{:?}", (fixed_dt - item).days());
        let mut need_remove = dt.year() != item.year();
        if need_remove {
            zfs::snapshots::remove("rpool/ROOT/ubuntu@version2");
        }
    }
    // Создаём новый
    zfs::snapshots::new(pool_name);
}

pub mod zfs {
    pub mod snapshots {
        extern crate chrono;
        use chrono::prelude::*;
        use std::process::Command; // https://github.com/chronotope/chrono

        #[cfg(target_os = "linux")]
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
            println!("{:?}", String::from_utf8(buffer.stdout));
            println!("{:?}", String::from_utf8(buffer.stderr));
        }

        #[cfg(not(target_os = "linux"))]
        pub fn new(pool_name: &str) {
            println!("I can't make are snapshot \"{}\" in this OS...", pool_name);
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
                .map(|s| {
                    chrono::DateTime::parse_from_str(&format!("{} +00:00", s), "%e_%m_%Y__%H_%M %z")
                        .unwrap()
                })
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
