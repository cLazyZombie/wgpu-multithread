#!/usr/bin/env python3
from http.server import HTTPServer, SimpleHTTPRequestHandler
import os

class CORSRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET')
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy',   'same-origin')
        return super(CORSRequestHandler, self).end_headers()

pwd = os.getcwd()
try:
    os.chdir("./web")  # or any path you like
    httpd = HTTPServer(('localhost', 8080), CORSRequestHandler)
    httpd.serve_forever()
finally:
    os.chdir(pwd)

