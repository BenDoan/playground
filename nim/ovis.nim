from osproc import execCmdEx
from re import findAll
from unicode import runes

proc get_cur_window_name(): string =
    var xprop_active_window = runes(execCmdEx("xprop -root 32x '\t$0' _NET_ACTIVE_WINDOW").output)

    #xprop_active_window.findAll(r"0x[0-9a-f]+")
    findAll(xprop_active_window, r"0x[0-9a-f]+", 0)

proc main() =
    echo(get_cur_window_name())


main()
