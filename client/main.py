import socket
import time

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 3306  # The port used by the server

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    counter = 0
    while True:
        counter = counter + 1
        s.sendall(b"Hello, server!")
        part = s.recv(4096)
        time.sleep(5)
        print("Received", repr(part))
        if counter > 2:
            break
    s.close()
