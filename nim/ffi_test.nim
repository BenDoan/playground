proc XScreenSaverQueryInfo(display, event_base, info): Status {.cdecl, dynlib: "libX11.so", importc.}

proc main() =
    echo "hello"

main()
