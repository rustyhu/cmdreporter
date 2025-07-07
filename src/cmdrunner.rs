use color_eyre::Result;
use std::thread;

pub struct CmdOutput {
    pub cmdname: String,
    pub summary: String,
}

pub fn collect(parallel: bool) -> Result<Vec<CmdOutput>> {
    let cmds_list = read_cmds_from_file("cmds.sh")?;

    // run cmds: default sequentially, optionally parallelly.
    let mut rst = Vec::new();
    if parallel {
        let handles: Vec<_> = cmds_list
            .iter()
            .map(|cmdline| {
                let cmdline = cmdline.clone();
                thread::spawn(move || run_single_cmd(&cmdline))
            })
            .collect();

        // from all threads
        for handle in handles {
            if let Ok(cmd_output) = handle.join() {
                rst.push(cmd_output);
            }
        }
    } else {
        for cmdline in cmds_list.iter() {
            rst.push(run_single_cmd(cmdline));
        }
    }
    Ok(rst)
}

fn read_cmds_from_file(filename: &str) -> Result<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let list = reader
        .lines()
        .filter_map(|line| {
            line.map(|l| {
                l.split_once('#')
                    .map(|(content, _comment)| content.trim())
                    .unwrap_or(l.trim())
                    .to_string()
            })
            .ok()
        })
        .filter(|line| !line.is_empty())
        .collect();

    Ok(list)
}

fn run_single_cmd(cmdline: &str) -> CmdOutput {
    println!("\x1b[34m Running [{cmdline}] ... \x1b[0m");
    let (cmd, args) = cmdline.trim().split_once(' ').unwrap_or((cmdline, ""));
    let info = if check_which_cmd(cmd) {
        run_cmd(cmd, args)
    } else {
        format!("CMD {cmd} not exist. Please recheck or install corresponding packages.")
    };

    CmdOutput {
        cmdname: cmd.into(),
        summary: info,
    }
}

use std::process::Command;

fn check_which_cmd(cmd: &str) -> bool {
    match Command::new("which").arg(cmd).output() {
        Ok(output) => output.status.success(),
        Err(_) => {
            println!("Command checker `which` may not exist, try running [{cmd}] directly;");
            true
        }
    }
}

fn run_cmd(cmd: &str, args: &str) -> String {
    let output = Command::new(cmd)
        .args(args.split_ascii_whitespace())
        .output();

    match output {
        Ok(output) => {
            // cmd output including TAB characters, leads to print error when showing in tabpages in Ratatui
            let mut info = String::from_utf8_lossy(&output.stdout).replace('\t', "    ");
            // possible stderr output
            if !output.status.success() {
                info += format!(
                    "\n\n----------\nCMD failed with status: {:?}, STDERR:\n{}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr)
                )
                .as_str();
            }
            info
        }
        Err(e) => e.to_string(),
    }
}
