# %%
import json
import socket
import struct

import numpy as np


HOST = "127.0.0.1"
PORT = 7878

# %%
msg = dict(SetBgColor=[0.5, 0.5, 0.5, 1.0])
msg = json.dumps(msg)
buf = bytes(msg, "utf-8")

# %%
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.send(struct.pack("!I", len(buf)))
    s.sendall(buf)
    t = s.recv(4)
    n = struct.unpack("!I", t)[0]
    m = s.recv(n)
    print(m)

# %%

msg = dict(SetShape=dict(Square=dict(size=0.2, ctr=dict(x=0.0, y=0.3))))
msg = dict(SetShape=dict(Circle=dict(radius=0.2, ctr=dict(x=0.0, y=0.3))))
# msg = dict( SetShape=dict(Cross=dict(size=0.2, ctr=dict(x=0.7, y=0.3), line_width=0.01)))
msg = json.dumps(msg)
buf = bytes(msg, "utf-8")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.send(struct.pack("!I", len(buf)))
    s.sendall(buf)
    t = s.recv(4)
    n = struct.unpack("!I", t)[0]
    m = s.recv(n)
    print(m)
# %%

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    msg = dict(SetBgColor=[0.5, 0.5, 0.5, 1.0])
    msg = json.dumps(msg)
    buf = bytes(msg, "utf-8")
    s.send(struct.pack("!I", len(buf)))
    s.sendall(buf)
    t = s.recv(4)
    n = struct.unpack("!I", t)[0]
    m = s.recv(n)
    print(m)

    msg = dict(SetShape=dict(Circle=dict(radius=0.2, ctr=dict(x=0.0, y=0.3))))
    # msg = dict( SetShape=dict(Cross=dict(size=0.2, ctr=dict(x=0.7, y=0.3), line_width=0.01)))
    msg = json.dumps(msg)
    buf = bytes(msg, "utf-8")
    s.send(struct.pack("!I", len(buf)))
    s.sendall(buf)
    t = s.recv(4)
    n = struct.unpack("!I", t)[0]
    m = s.recv(n)
    print(m)

# %%

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    msg = dict(SetBgColor=[0.5, 0.5, 0.5, 1.0])
    msg = json.dumps(msg)
    buf = bytes(msg, "utf-8")
    for i in range(100):
        s.send(struct.pack("!I", len(buf)))
        s.sendall(buf)
        t = s.recv(4)
        n = struct.unpack("!I", t)[0]
        m = s.recv(n)
        print(m)

# %%

x = np.arange(-1, 1.1, 0.1)
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    for jj in x:
        for ii in x:
            msg = dict(SetShape=dict(Circle=dict(radius=0.2, ctr=dict(x=ii, y=jj))))
            msg = json.dumps(msg)
            buf = bytes(msg, "utf-8")
            s.send(struct.pack("!I", len(buf)))
            s.sendall(buf)
            t = s.recv(4)
            n = struct.unpack("!I", t)[0]
            m = s.recv(n)
            print(m)
# %%
