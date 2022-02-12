import json
import socket


HOST = "127.0.0.1"
PORT = 7878

msg = dict(SetShape=dict(Square=dict(size=0.2,ctr=dict(x=0.0,y=0.0))))
msg = json.dumps(msg)
buf = bytes(msg, 'utf-8')

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(buf)