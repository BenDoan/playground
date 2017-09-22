package me.bendoan.helloKotlin;

import java.util.Random;

public class Nulls {
    public static void main(String[] args) {
        User user = getUser();

        System.out.printf("Length of name is %s", user.getName().length());

        // should really be:

        if (user != null) {
            if (user.getName() != null) {
                System.out.printf("Len is %s", user.getName().length());
            }
        }
    }

    private static User getUser(){
        if (new Random().nextBoolean()){
            return new User("Ben", 22);
        } else {
            return null;
        }
    }
}
