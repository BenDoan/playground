package me.bendoan.grammar

import java.io.File;

data class NumOccurrences(var numIe: Int, var numEi: Int, var numIeIncorrect: Int, var numEiIncorrect: Int)

fun main(args: Array<String>) {
    val words = File("/usr/share/dict/american-english")
        .readLines()
        .map({ it.trim() })
        .filter({ !it.endsWith("'s") })
    val numOccurrences = calcNumOccurrences(words)
    println("""
        |#IE:             ${numOccurrences.numIe}
        |#IE after c:     ${numOccurrences.numIeIncorrect}
        |
        |#EI:             ${numOccurrences.numEi}
        |#EI not after c: ${numOccurrences.numEiIncorrect}""".trimMargin())
}

fun calcNumOccurrences(words: List<String>): NumOccurrences {
    var counter = NumOccurrences(0, 0, 0, 0)

    for (word in words) {
        for (i in 0 until word.length) {
            val prevChar = if (i == 0) '0' else word.get(i-1)
            val firstChar = word.get(i)
            val secondChar = if (i == word.length-1) '0' else word.get(i+1)

            val combined = charArrayOf(firstChar, secondChar).joinToString(separator="")

            if (combined == "ei") {
                counter.numEi++
                if (prevChar != 'c') counter.numEiIncorrect++
            } else if (combined == "ie") {
                counter.numIe++
                if (prevChar == 'c') counter.numIeIncorrect++
            }
        }
    }

    return counter
}
