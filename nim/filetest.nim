proc main =
    var name = "test.txt"

    var f = open(name, fmAppend)

    f.write("Hello")

main()
