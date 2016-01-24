function get_request(site, path, fun, method, port)
    local method = method
    if not method then
        method = "GET"
    end

    local port = port
    if not port then
        port = 80
    end

    conn = net.createConnection(net.TCP, 0)
    conn:on("receive", fun)

    conn:dns(site, function(conn2, ip)
        conn:connect(port, ip)
        conn:send("GET "..path.." HTTP/1.1\r\nHost: "..site.."\r\nConnection: keep-alive\r\nAccept: */*\r\n\r\n")
    end)
end
get_request("flainted.com", "/", function(conn, payload)
    print(payload)
end, "GET", 5000)
