# %%
import json
import socket


HOST = "127.0.0.1"
PORT = 7878

# %%
msg = dict(SetBgColor=[0.5, 0.5, 0.5, 1.0])
msg = json.dumps(msg)
buf = bytes(msg, "utf-8")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(buf)

# %%

msg = dict(SetShape=dict(Square=dict(size=0.2, ctr=dict(x=0.0, y=0.3))))
msg = dict(SetShape=dict(Circle=dict(radius=0.2, ctr=dict(x=0.0, y=0.3))))
# msg = dict( SetShape=dict(Cross=dict(size=0.2, ctr=dict(x=0.7, y=0.3), line_width=0.01)))
msg = json.dumps(msg)
buf = bytes(msg, "utf-8")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)
# %%

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    msg = dict(SetBgColor=[0.5, 0.5, 0.5, 1.0])
    msg = json.dumps(msg)
    buf = bytes(msg, "utf-8")
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)

    msg = dict(SetShape=dict(Circle=dict(radius=0.2, ctr=dict(x=0.0, y=0.3))))
    # msg = dict( SetShape=dict(Cross=dict(size=0.2, ctr=dict(x=0.7, y=0.3), line_width=0.01)))
    msg = json.dumps(msg)
    buf = bytes(msg, "utf-8")
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)
