import socket
import time

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 3306  # The port used by the server

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    counter = 0
    while True:
        counter = counter + 1

        command = input("iotadb> ")

        s.sendall(str.encode(command))
        part = s.recv(4096)
        time.sleep(0.25)
        print("Received", repr(part))
        if counter > 2:
            break
    print("Closing connection to server.")
    s.close()
