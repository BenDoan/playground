import std.stdio;
import std.process;
import std.regex;
import std.string;
import std.c.time;
import std.datetime;
import std.file;
import std.getopt;
import std.conv;

import core.thread;

import std.c.stdlib;

alias core.thread.Thread.sleep  Sleep;

const string DEFAULT_LOG_LOCATION = "/home/ben/.ovis-log";
const string DEFAULT_LOG_TYPE = "default";
const auto SLEEP_DELAY = 50.msecs;
const uint DEFAULT_MIN_IDLE_TIME = 300; // 5 mins in seconds
const int INVALID_ARGUMENTS = 1;
const string HELP_MESSAGE = "Usage: ovis [OPTION]...
options:
--output           defines where the ovis log is placed
--log-type         defines which type of log is outputted (opts: default, csv)
--verbose          turns on verbose logging
--min-idle-time    minimum seconds of :idle time before tracking turns off
--help             prints this help message";

string logLocation = DEFAULT_LOG_LOCATION;
string logType = DEFAULT_LOG_TYPE;
bool verbose = false;
uint minIdleTime = DEFAULT_MIN_IDLE_TIME;
bool help = false;

int main(string[] args){
    try {
        getopt(args,
            std.getopt.config.bundling,
            "output|o", &logLocation,
            "log-type|l", &logType,
            "verbose|v", &verbose,
            "min-idle-time|m", &minIdleTime,
            "help|h", &help);
    }catch(GetOptException e){
        writeln("Invalid Arguments");
        writeln(HELP_MESSAGE);
        return INVALID_ARGUMENTS;
    }

    if (help){
        writeln(HELP_MESSAGE);
    }else{
        track_time(logLocation, logType, minIdleTime, verbose);
    }

    return 0;
}

void track_time(string logLocation, string logType, uint minIdleTime, bool verbose){
    string lastWindow = "";
    auto lastTime = Clock.currTime();

    while(true){
        auto logFile = File(logLocation, "a");
        string curWindow = get_cur_window_name();

        if (curWindow != null && lastWindow != curWindow && lastWindow != ""){
            auto currentTime = Clock.currTime();


            int idleTime = get_idle_time();
            if (idleTime < (minIdleTime*1000)){
                if (verbose){
                    writeln(lastWindow);
                    writeln(currentTime - lastTime);
                    writeln("");
                }

                switch(logType){
                    case "default":
                        writeLogDefault(lastWindow, currentTime, lastTime, logFile);
                        break;
                    default:
                        break;
                }

            }else{
                if (verbose){
                    writef("User has been idle for %s ms - not logging time\n", idleTime);
                }
            }

            lastTime = currentTime;
        }

        lastWindow = curWindow;
        Sleep(SLEEP_DELAY);
    }
}

auto writeLogDefault(string lastWindow, SysTime currentTime, SysTime lastTime, File logFile){
    string outstr = format("Changing to %s, %s\n", lastWindow, currentTime - lastTime);
    logFile.write(outstr);
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

auto xprintidle_on_system(){
    try {
        string which_xprintidle = shell("which xprintidle");
        return true;
    }catch(std.exception.ErrnoException e){
        return false;
    }
}
