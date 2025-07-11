# cmdreporter

A command line result viewer, built with Ratatui.

## Usage

Write the commands you want to run in `cmds.sh` file, for example: 
```bash
ls -l
uname -a
echo hello world
```

Run `cmdreporter` to execute these in batch, sequentially by default. Add option `-p`/`--parallel` to run them in parallel.