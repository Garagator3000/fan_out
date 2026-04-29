import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

for i in range(100_000):
    msg = str(i).encode()
    sock.sendto(msg, ("127.0.0.1", 3000))

    if i % 1000 == 0:
        print(f"sent {i}")
