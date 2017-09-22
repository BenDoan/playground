package me.bendoan.helloKotlin

import java.util.*

fun main(args: Array<String>){
    val user: User = getUser()
    println("Length of name is ${user.name.length}")






























    val user2 = getUser2()

    // println("Length of name is ${user2.name.length}")

    if (user2 != null) {
        println("Length of name is ${user2.name.length}")
    }

    println("Length of name is ${user2?.name?.length}")

    println("Length of name is ${user2?.name?.length ?: "<undefined>"}")

    // For NPE-lovers
    println("Length of name is ${user2!!.name.length}")
}


























fun getUser(): User {
    return User("Ben", 22)
}

fun getUser2(): UserJava? {
    return if (Random().nextBoolean()){
        UserJava("Ben", 22)
    } else {
        null
    }
}