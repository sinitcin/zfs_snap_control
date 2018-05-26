extern crate chrono;
extern crate clap;
use chrono::prelude::*;
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new(
        "     
-------------------------------------------------------------------------------
   mmm   m   m  m   m  m   m  m   m  m    m m   m         mmmm   m   m   mmm  
  #\"  \"  #  ##  #   #  #  ##  #   #  #    # #   #         #\" \"#  \"m m\"  #\"  \" 
  #      # # #  #\"\"\"#  # # #  #   #  #\"\"m # #\"\"\"#         #   #   #m#   #     
  \"#mm\"  #\"  #  #   #  #\"  #  #mmm#m #mm\" # #   #    #    ##m#\"   \"#    \"#mm\" 
                                   #                      #       m\"          
                                                          \"      \"\"         
------------------------------ ZFS Snap Control -------------------------------
    ",
    ).about("Create ZFS snapshots on a schedule and automatically delete old ones!")
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
        if (fixed_dt - item).num_days() > days_duration {
            zfs::snapshots::remove(&format!(
                "{}@{}_{}_{}__{}_{}",
                pool_name,
                item.day(),
                item.month(),
                item.year(),
                item.hour(),
                item.minute()
            ));
        }
    }
    // Создаём новый
    zfs::snapshots::new(pool_name);
}

#[cfg(test)]
mod test {
    extern crate chrono;
    use super::zfs;
    #[cfg(target_os = "linux")]
    use chrono::prelude::*;

    #[test]
    fn make_snaps() {
        zfs::snapshots::new("rpool/ROOT/ubuntu");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn remove_snaps() {
        let pool_name = "rpool/ROOT/ubuntu";
        let list = zfs::snapshots::list(pool_name);
        for item in list {
            zfs::snapshots::remove(&format!(
                "{}@{}_{}_{}__{}_{}",
                pool_name,
                item.day(),
                item.month(),
                item.year(),
                item.hour(),
                item.minute()
            ));
        }
    }
}

/// Модуль zfs в данной версии позволяет создавать, удалять и просматривать список снапшотов.
///
/// # Examples
/// Пример для создания снапшота
/// ```
/// zfs::snapshots::new("rpool/ROOT/ubuntu@version1");
/// ```
///
/// Пример для просмотра списка снапшотов
/// ```
/// let list = zfs::snapshots::list(pool_name);
/// for item in list {
///     ...
/// }
/// ```
/// Пример для удаления снапшота
/// ```
/// zfs::snapshots::new("rpool/ROOT/ubuntu@23_5_2018__17_50");
/// ```
pub mod zfs {
    pub mod snapshots {

        extern crate chrono;
        #[cfg(target_os = "linux")]
        use chrono::prelude::*;
        use std::process::Command;
        use std::str;

        /// Создание нового снапшота в ZFS
        ///
        /// Указывается имя пула, стандартное для ZFS, например:
        /// ```
        /// rpool/ROOT/ubuntu
        /// rpool/ROOT/ubuntu
        /// ...
        /// ```
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
            let err_message = "Не смог создать снапшот";
            let mut cmd = Command::new("zfs");
            cmd.args(&["snapshot", &format!("{}@{}", pool_name, snap_name)]);

            let status = cmd.status().expect(err_message);
            assert!(status.success());

            let output = cmd.output().expect(err_message);
            println!("{}", str::from_utf8(&output.stdout).unwrap());
            println!("{}", str::from_utf8(&output.stderr).unwrap());
        }

        /// Создание нового снапшота в ZFS
        ///
        /// Указывается имя пула, стандартное для ZFS, например:
        /// ```
        /// rpool/ROOT/ubuntu
        /// rpool/ROOT/ubuntu
        /// ...
        /// ```
        #[cfg(not(target_os = "linux"))]
        pub fn new(pool_name: &str) {
            println!("I can't make are snapshot \"{}\" in this OS...", pool_name);
        }

        /// Список дат когда были сделаны снапшоты
        ///
        /// Указывается имя пула, стандартное для ZFS, например:
        /// ```
        /// rpool/ROOT/ubuntu
        /// rpool/ROOT/ubuntu
        /// ...
        /// ```
        pub fn list(pool_name: &str) -> Vec<chrono::DateTime<chrono::FixedOffset>> {
            let err_message = "Не могу получить список снапшотов";

            let mut cmd = Command::new("zfs");
            cmd.args(&["list", "-t", "snapshot"]);
            let status = cmd.status().expect(err_message);
            assert!(status.success());

            let output = cmd.output().expect(err_message);

            // Получаем список снапшотов сделанных в pool_name именно нами
            String::from_utf8_lossy(&output.stdout)
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

        /// Удаление снашота по полному имени
        ///
        /// Имя снапшота должно быть указано полностью:
        /// ```
        /// rpool/ROOT/ubuntu@version1
        /// rpool/ROOT/ubuntu@23_5_2018__17_50
        /// ...
        /// ```
        pub fn remove(full_snap_name: &str) {
            let err_message = "Не могу уничтожить снапшот";
            let mut cmd = Command::new("zfs");
            cmd.args(&["destroy", &full_snap_name.to_owned()]);
            let status = cmd.status().expect(err_message);
            assert!(status.success());
            cmd.output().expect(err_message);
        }
    }
}
