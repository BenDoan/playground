import std.stdio;
import std.process;
import std.regex;
import std.string;
import std.c.time;
import core.thread;
import std.datetime;
import std.file;
import std.getopt;
import std.conv;

alias core.thread.Thread.sleep  Sleep;

const string DEFAULT_LOG_LOCATION = "/home/ben/.ovis-log";
const auto SLEEP_DELAY = 50.msecs;
const int INVALID_ARGUMENTS = 1;
const string HELP_MESSAGE = "Usage: ovis [OPTION]...
options:
--log-location     defines where the ovis log is placed
--verbose          turns on verbose logging
--help             prints this help message";

string logLocation = DEFAULT_LOG_LOCATION;
bool verbose = false;
bool help = false;

int main(string[] args){
    try {
        getopt(args,
            std.getopt.config.bundling,
            "log-location|l", &logLocation,
            "verbose|v", &verbose,
            "help|h", &help);
    }catch(GetOptException e){
        writeln("Invalid Arguments");
        writeln(HELP_MESSAGE);
        return INVALID_ARGUMENTS;
    }

    if (help){
        writeln(HELP_MESSAGE);
    }else{
        track_time(logLocation, verbose);
    }

    return 0;
}

void track_time(string logLocation, bool verbose){
    string lastWindow = "";
    auto lastTime = Clock.currTime();
    uint idleTime = 0;

    while(true){
        string curWindow = get_cur_window_name();

        if (curWindow != null && lastWindow != curWindow && lastWindow != ""){
            auto currentTime = Clock.currTime();

            if (verbose){
                writeln(lastWindow);
                writeln(currentTime - lastTime);
                writeln("");
            }

            string outstr = format("Changing to %s, %s\n", lastWindow, currentTime - lastTime);

            auto f = File(logLocation, "a");
            f.write(outstr);
            lastTime = currentTime;
            idleTime = get_idle_time();
        }

        lastWindow = curWindow;
        Sleep(SLEEP_DELAY);
    }
}

auto get_cur_window_name(){
    string xprop_active = shell("xprop -root 32x '\t$0' _NET_ACTIVE_WINDOW");

    auto m = match(xprop_active, r"(0x[0-9a-f]+)");
    if (!m){
        return null;
    }
    string window_id = m.captures[0];

    string net_wm_name = shell(format("xprop -id %s _NET_WM_NAME", window_id));
    m = match(net_wm_name, r"=(.*)");
    if (!m){
        return null;
    }
    auto wm_name = m.captures[0][3..$-1];
    return wm_name;
}

auto get_idle_time(){
    string time_idle = shell("xprintidle");

    return parse!uint(time_idle);
}
