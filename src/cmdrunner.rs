use color_eyre::Result;

pub struct CmdOutput {
    pub cmdname: String,
    pub summary: String,
}

// current let's collect static data first
pub fn collect() -> Result<Vec<CmdOutput>> {
    // Read the commands list from a file
    let cmds_list = read_cmds_from_file("cmds.sh")?;

    // executed sequentially, not concurrently.
    let mut rst = Vec::new();
    for cmdline in cmds_list.iter() {
        let (cmd, args) = cmdline.trim().split_once(' ').unwrap_or((cmdline, ""));

        let info = if check_which_cmd(cmd) {
            run_cmd(cmd, args)
        } else {
            format!("CMD {cmd} not exist. Please recheck or install corresponding packages.")
        };

        rst.push(CmdOutput {
            cmdname: cmd.into(),
            summary: info,
        });
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
