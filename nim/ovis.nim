import osproc
import re
import subexes
import os
import times
import strutils

var LOG_NAME = expandTilde("~/.novis-log")
var SLEEP_TIME = 500

proc get_cur_window_name(): string =
    let xprop_active_window = osproc.execCmdEx("xprop -root 32x '\t$0' _NET_ACTIVE_WINDOW").output
    let window_id =  xprop_active_window.findAll(re"0x[0-9a-f]+")[0]

    let net_wm_name = osproc.execCmdEx(subex"xprop -id $1 _NET_WM_NAME" % window_id).output;

    net_wm_name[29.. -3]

proc main() =
    var log_file = open(LOG_NAME, fmAppend)
    var last_window = ""
    var last_time = times.getTime()

    while true:
        let cur_window = get_cur_window_name()

        if last_window != cur_window and last_window != "":
            let time = times.getTime()
            var time_spent = time-last_time

            var out_str = format("$1, $2 seconds\n", last_window, time_spent)
            log_file.write(out_str)
            echo out_str

            last_time = time

        last_window = cur_window
        os.sleep(SLEEP_TIME)

main()
