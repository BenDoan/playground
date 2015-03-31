package main

import (
	"encoding/json"
	"fmt"
	"github.com/op/go-logging"
	"io/ioutil"
	"os"
	"os/exec"
	"regexp"
	"time"
)

var log = logging.MustGetLogger("Govis")
var format = logging.MustStringFormatter("%{color}[%{id:03x}] %{time:15:04:05} %{level:.4s}%{color:reset} %{message}")

var configFileName = "govis.json"

func main() {
	backend := logging.NewLogBackend(os.Stderr, "", 0)
	backendFormatter := logging.NewBackendFormatter(backend, format)
	logging.SetBackend(backendFormatter)

	c := JsonCfg{}
	fmt.Println(c.GetConfigFile(configFileName))

	TrackTime()
}

type JsonCfg struct {
	TickInterval int
	LogName      string
}

func (I *JsonCfg) GetConfigFile(path string) *JsonCfg {
	b, err := ioutil.ReadFile(path)

	if err != nil {
		log.Error("%s", err)
	}

	err = json.Unmarshal(b, &I)

	if err != nil {
		log.Error("%s", err)
	}
	return I
}

func TrackTime() {
	lastTime := time.Now()
	lastWindow := GetCurrentWindowName()

	c := time.Tick(400 * time.Millisecond)
	for now := range c {
		currentWindow := GetCurrentWindowName()

		if currentWindow != lastWindow {
			timeDiff := time.Now().Sub(lastTime)
			fmt.Printf("Changed from %s to %s for %s", lastWindow, currentWindow, timeDiff)
		}

		lastWindow = currentWindow
		var _ = now
	}
}

func GetCurrentWindowID() string {
	out, err := exec.Command("xprop", "-root", "_NET_ACTIVE_WINDOW").Output()

	if err != nil {
		log.Warning("Could get current window id: %s", err)
		return ""
	}

	re := regexp.MustCompile(`0x[a-f0-9]+`)
	match := re.FindStringSubmatch(string(out))

	if len(match) < 1 {
		log.Warning("Could get current window id")
		return ""
	}

	return match[0]
}

func GetCurrentWindowName() string {
	out, err := exec.Command("xprop", "-id", GetCurrentWindowID(), "_NET_WM_NAME").Output()

	if err != nil {
		log.Warning("Could get current window name: %s", err)
		return ""
	}

	re := regexp.MustCompile(`"(.*)"`)
	match := re.FindStringSubmatch(string(out))

	if len(match) < 2 {
		log.Warning("Could get current window name")
		return ""
	}

	return match[1]
}
