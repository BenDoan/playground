package me.bendoan.helloKotlin

data class User(var name: String, var age: Int)

fun main(args: Array<String>) {
    val ben = User("Ben Doan", 22)
    val ben2 = User("Ben Doan", 22)

    val nick = User("Nick Richman", 21)

    println("Does $ben equal $nick? ${ben == nick}")
    println("Does $ben equal $ben2? ${ben == ben2}")

}