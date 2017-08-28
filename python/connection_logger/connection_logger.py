#!/usr/bin/env python2
import BaseHTTPServer

import datetime

LOGFILE = "connections.log"
PORT = 8070

class Handler(BaseHTTPServer.BaseHTTPRequestHandler) :
    def do_HEAD(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()
    def do_GET(self):
        with open(LOGFILE, "a+") as f:
            f.write("%s\n" % datetime.datetime.now())
        self.send_response(200, "ok")
        self.end_headers()


server = BaseHTTPServer.HTTPServer(("0.0.0.0", PORT),Handler)

print "Listening on :%s" % PORT
try :
    server.serve_forever()
except KeyboardInterrupt:
    pass
server.server_close()
