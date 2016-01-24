if file.list()["wifilogin.txt"] then
    file.open("wifilogin.txt", "r")
    login = file.readline()

    ssid = nil
    pass = nil
    for a, b in string.gmatch(login, "(%w+):(%w+)") do
        ssid = a
        pass = b
    end

    print("Trying to login with |"..ssid.."|:|"..pass.."|")
    wifi.setmode(wifi.STATION)
    wifi.sta.config(ssid, pass)
    wifi.sta.connect()
    tmr.alarm(1, 1000, 1, function()
        if wifi.sta.getip()== nil then
            print("IP unavailable, Waiting...")
        else
            tmr.stop(1)
            print("The module MAC address is: " .. wifi.ap.getmac())
            print("Config done, IP is "..wifi.sta.getip())
        end
    end)
else
    print("Couldn't find wifilogin.txt")
end

tmr.alarm(2, 6000, 0, function()
    if file.list()["main.lua"] then
        print("Running main.lua")
        dofile("main.lua")
    else
        print("Couldn't find main file")
    end
end)
