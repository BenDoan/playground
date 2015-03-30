package main

import (
	"fmt"
	"github.com/op/go-logging"
	"os"
	"os/exec"
	"regexp"
)

var log = logging.MustGetLogger("Govis")
var format = logging.MustStringFormatter("%{color}[%{id:03x}] %{time:15:04:05} %{level:.4s}%{color:reset} %{message}")

func main() {
	backend := logging.NewLogBackend(os.Stderr, "", 0)
	backendFormatter := logging.NewBackendFormatter(backend, format)
	logging.SetBackend(backendFormatter)

	fmt.Println(GetCurrentWindowName())
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

	fmt.Println(string(out))

	re := regexp.MustCompile(`"(.*)"`)
	match := re.FindStringSubmatch(string(out))

	if len(match) < 2 {
		log.Warning("Could get current window name")
		return ""
	}

	return match[1]
}
