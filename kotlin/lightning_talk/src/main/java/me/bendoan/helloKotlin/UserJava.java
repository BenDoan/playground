package me.bendoan.helloKotlin;

public class UserJava {
    private String name;
    private int age;

    public UserJava(String name, int age) {
        this.name = name;
        this.age = age;
    }

    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;

        UserJava userJava = (UserJava) o;

        if (age != userJava.age) return false;
        return name != null ? name.equals(userJava.name) : userJava.name == null;
    }

    @Override
    public int hashCode() {
        int result = name != null ? name.hashCode() : 0;
        result = 31 * result + age;
        return result;
    }

    @Override
    public String toString() {
        return "UserJava{" +
                "name='" + name + '\'' +
                ", age=" + age +
                '}';
    }
}

class UserRunner {
    public static void main(String[] args) {
        UserJava ben = new UserJava("Ben Doan", 22);
        UserJava ben2 = new UserJava("Ben Doan", 22);

        UserJava nick = new UserJava("Nick Richman", 21);

        System.out.printf("Does %s == %s? %s\n", ben, nick, ben.equals(nick));
        System.out.printf("Does %s == %s? %s\n", ben, ben, ben.equals(ben));
    }
}
