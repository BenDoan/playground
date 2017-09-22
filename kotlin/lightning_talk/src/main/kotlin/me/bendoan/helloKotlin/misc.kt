package me.bendoan.helloKotlin

import java.io.Closeable
import java.io.File
import java.io.FileOutputStream
import java.time.Instant

fun main(args: Array<String>) {

    // COLLECTION LITERALS
    val nums = listOf(1, 2, 3, 4)
    val morseCode = mapOf("a" to ".-",
                          "b" to "-...",
                          "c" to "-.-.")


























































    // MULTI-LINE STRINGS
    println("""item1
      item2
      item3
      item4""")


































    // TUPLE UNPACKING
    val coords = Pair(10, 20)

    val (xCoord, yCoord) = coords
    println("x: $xCoord, y: $yCoord")

    val ben = User("Ben", 22)
    val (name, age) = ben





































    // HIGHER ORDER FUNCTIONS
    val names = listOf("sam", "joe", "john", "jan")
    val uppercaseJNamesByLastCharacter: Map<Char, List<String>> = names
            .filter {x -> x.startsWith("j") }
            .map { it.toUpperCase() }
            .groupBy { it.last() }

    // uppercaseJNamesByLastCharacter = {"N": ["JOHN", "JAN"], "E": ["JOE"]}



































    // FOR LOOP
    for (num in 1..100){
        println(num)
    }

    // PATTERN MATCHING
    val inp: Any = "something"
    val interpreted = when (inp) {
        1 -> "is one"
        "something" -> "is something"
        is String -> "is a string"
        in 0..10 -> "is a small number"
        {it: User -> it.name == "Ben"} -> "is ben"
        else -> "No idea"
    }





















































    // SMART CASTING
    val obj: Any = "hello"

    // println(obj.length)

    // the java way
    if (obj is String) {
        val strObj: String = obj as String
        println(strObj.length)
    }

    // the kotlin way
    if (obj is String) {
        println(obj.length)
    }

































    // EXTENSION FUNCTIONS
    fun String.asDate(): Instant {
        return java.time.Instant.parse(this)
    }

    var date: Instant = "2007-12-03T10:15:30.00Z".asDate()












































    // DEFAULT PARAMETERS
    fun greet(name: String, greeting: String = "Hello") {
        println("$greeting $name")
    }

    greet("Ben")
    greet("Ben", "Hallo")
    greet("Ben", greeting = "Bonjour")

    // vs java style of

    fun greet2(name: String, greeting: String) {
        println("$greeting $name")
    }
    fun greet2(name: String) {
        greet2(name, "Hello")
    }
















































    // IF STATEMENTS ARE EXPRESSIONS
    var argsState = if (args.isEmpty()) {
        "IsEmpty"
    } else {
        "NotEmpty"
    }



    // vs java style of:

    var argState2: String
    if (args.isEmpty()){
        argState2 = "IsEmpty"
    } else {
        argState2 = "NotEmpty"
    }

    // can also collapse further

    argsState = if (args.isEmpty()) "IsEmpty" else "NotEmpty"































    fun with(obj: Closeable, body: () -> Unit) {
        try {
            body()
        } finally {
            obj.close()
        }
    }

    val fos = FileOutputStream("/a/file/path")
    with(fos) {
        fos.write(1)
    }





























    // HTML DSL
//    val dropdown = li {
//        classes = setOf("dropdown")
//
//        a("#", null) {
//            classes = setOf("dropdown-toggle")
//            attributes["data-toggle"] = "dropdown"
//            role = "button"
//            attributes["aria-expanded"] = "false"
//
//            ul {
//                classes = setOf("dropdown-menu")
//                role = "menu"
//
//                li { a("#") { +"Action" } }
//                li { a("#") { +"Another action" } }
//                li { a("#") { +"Something else here" } }
//                li { classes = setOf("divider")}
//                li { classes = setOf("dropdown-header"); +"Nav header" }
//                li { a("#") { +"Separated link" } }
//                li { a("#") { +"One more separated link" } }
//            }
//
//            span {
//                classes = setOf("caret")
//            }
//        }
//    }

















}
