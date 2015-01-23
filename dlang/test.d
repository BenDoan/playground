import std.stdio;
import core.thread;
import std.c.stdlib;

void main(){
    auto logFile = File("/home/ben/.ovis-log", "a");

    while(true){
        logFile.write("Test line\n");
        core.thread.Thread.sleep(1.seconds);
    }
}
