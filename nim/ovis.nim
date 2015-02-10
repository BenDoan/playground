let doc = """
Tracks time using the currently active window.

Usage:
    ovis [options]

    Options:
      --output          defines where the ovis log is placed
      --log-type        defines the log type
      --verbose         turns on verbose logging on
      --min-idle-time   minimum seconds of idle time before tracking turns off
      --help            prints this help message
"""

import docopt
import os
import osproc
import re
import times

proc get_cur_window_name(): string
proc write_entry(last_window: string, time_spent: int, log_file: File)
proc get_idle_time(): int

let args = docopt(doc, version="0.0.1")

if args["--output"]:
    let LOG_NAME = expandTilde(args["--output"])
else:
    let LOG_NAME = expandTilde("~/.novis-log")

if args["--min-idle-time"]:
    let MIN_IDLE_TIME = args["--min-idle-time"]
else:
    let MIN_IDLE_TIME = 500000 # 5 minutes


let SLEEP_TIME = 500

var log_file = open(LOG_NAME, fmAppend)
var last_window = ""
var last_time = times.getTime()

proc main() =
    while true:
        let cur_window = get_cur_window_name()

        if last_window != cur_window and last_window != "":
            let time = times.getTime()
            let time_spent = time-last_time
            write_entry(last_window, time_spent, log_file)

            last_time = time

        last_window = cur_window
        os.sleep(SLEEP_TIME)

proc write_entry(last_window: string, time_spent: int, log_file: File) =
    let out_str = format("$#, $# seconds\n", last_window, time_spent)
    log_file.write(out_str)
    echo out_str

proc get_cur_window_name(): string =
    let xprop_active_window = osproc.execCmdEx("xprop -root 32x '\t$0' _NET_ACTIVE_WINDOW").output
    let window_id =  xprop_active_window.findAll(re"0x[0-9a-f]+")[0]

    let net_wm_name = osproc.execCmdEx(format("xprop -id $1 _NET_WM_NAME", window_id)).output;

    net_wm_name[29.. -3]

proc get_idle_time(): int =
    parseInt(osproc.execCmdEx("xprintidle").output)

proc leave() {.noconv.} =
    let time_spent = times.getTime()-last_time
    write_entry(last_window, time_spent, log_file)

    quit 0

setControlCHook(leave)
addQuitProc(leave)

main()
