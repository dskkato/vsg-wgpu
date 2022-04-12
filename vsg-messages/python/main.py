# %%
import socket
import struct
import message_pb2
import shapes_pb2

addr = ("localhost", 7878)
# %%
bg_color = message_pb2.BgColor(color=[0.0, 0.0, 0.2, 1.0])
message = message_pb2.RootMessage(set_bg_color=bg_color)
buf = message.SerializeToString()
n = struct.pack(">i", len(buf))

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(addr)
    s.sendall(n)
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)
# %%
ctr = shapes_pb2.Coordinates(x=0.1,y =-0.5)
square = shapes_pb2.Square(size=0.2, ctr=ctr)
shape = shapes_pb2.Shape(square=square)
message = message_pb2.RootMessage(set_shape=shape)
buf = message.SerializeToString()
n = struct.pack(">i", len(buf))

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(addr)
    s.sendall(n)
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)
# %%
ctr = shapes_pb2.Coordinates(x=0.1,y =-0.5)
with open("macaque.jpg", "rb") as f:
    data = f.read()
tex = message_pb2.Texture(index=0, data=data)
message = message_pb2.RootMessage(set_texture=tex)
buf = message.SerializeToString()
n = struct.pack(">i", len(buf))

addr = ("localhost", 7878)
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(addr)
    s.sendall(n)
    s.sendall(buf)
    ret = s.recv(1024)
    print(ret)
# %%
