# Perf 60s cmds list
# Follow the guide of [Linux Performance Analysis in 60,000 Milliseconds](https://www.brendangregg.com/Articles/Netflix_Linux_Perf_Analysis_60s.pdf).

uptime
# dmesg | tail
vmstat 1 3        # 1 delay 3 count
mpstat -P ALL 1 3 # 1 delay 3 count
pidstat 1 3       # 1 delay 3 count
iostat -xz 1 3    # 1 delay 3 count
free -m
sar -n DEV 1 2
sar -n TCP,ETCP 1 2
top -b -n 1
