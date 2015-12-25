function get_request(site, path, fun)
    conn = net.createConnection(net.TCP, 0)
    conn:on("receive", fun)

    conn:dns(site, function(conn2, ip)
        conn:connect(80, ip)
        conn:send("GET "..path.." HTTP/1.1\r\nHost: "..site.."\r\nConnection: keep-alive\r\nAccept: */*\r\n\r\n")
    end)

end

get_request("www.bendoan.me", "/", function(conn, payload)
    print(payload)
end)
