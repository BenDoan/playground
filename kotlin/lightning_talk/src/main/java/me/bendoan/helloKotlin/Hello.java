package me.bendoan.helloKotlin;

import java.util.NoSuchElementException;
import java.util.Scanner;

public class Hello {
    public static void main(String[] args) {
        System.out.println("What's your name");

        String name = "";
        try {
            Scanner scanner = new Scanner(System.in);
            name = scanner.nextLine();
        } catch (NoSuchElementException e){

        }

        System.out.printf("Hello %s", name);
    }
}
