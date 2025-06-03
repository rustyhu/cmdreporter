//! Follow the guide of [Linux Performance Analysis in 60,000 Milliseconds](https://www.brendangregg.com/Articles/Netflix_Linux_Perf_Analysis_60s.pdf).
//!
//! Commands list:
//! - uptime
//! - dmesg | tail
//! - vmstat 1
//! - mpstat -P ALL 1
//! - pidstat 1
//! - iostat -xz 1
//! - free -m
//! - sar -n DEV 1
//! - sar -n TCP,ETCP 1
//! - top

use std::process::Command;

// should be executed sequentially, not concurrently.
const CMDS_LIST: [&str; 9] = [
    "uptime",
    // "dmesg | tail",
    "vmstat 1 3",        // 1 delay 3 count
    "mpstat -P ALL 1 3", // 1 delay 3 count
    "pidstat 1 3",       // 1 delay 3 count
    "iostat -xz 1 3",    // 1 delay 3 count
    "free -m",
    "sar -n DEV 1 2",
    "sar -n TCP,ETCP 1 2",
    "top -b -n 1",
];

pub struct Report {
    pub cmdname: String,
    pub summary: String,
}

// current let's collect static data first
pub fn collect() -> Vec<Report> {
    let mut rst = Vec::new();
    for cmdline in CMDS_LIST.iter() {
        let (cmd, args);
        match cmdline.trim().split_once(' ') {
            Some((c, a)) => {
                cmd = c;
                args = a;
            }
            None => {
                cmd = cmdline;
                args = "";
            }
        }

        let info = if whether_cmd_exist(cmd) {
            // format!("Get CMD {}! args - \"{}\"\n", cmd, args)
            run_cmd(cmd, args)
        } else {
            format!("CMD {cmd} not exist. Please recheck or install corresponding packages.")
        };

        rst.push(Report {
            cmdname: cmd.into(),
            summary: info,
        });
    }
    rst
    // run_cmd("top", "-b -n 1")
}

fn whether_cmd_exist(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .expect("Failed to execute 'type' process")
        .status
        .success()
}

fn run_cmd(cmd: &str, args: &str) -> String {
    let output = Command::new(cmd)
        .args(args.split_ascii_whitespace())
        .output()
        .expect(&format!("Failed to execute command `{}`", cmd));
    // cmd output including TAB characters, leads to print error when showing in tabpages in Ratatui
    String::from_utf8_lossy(&output.stdout).replace('\t', "    ")
}
