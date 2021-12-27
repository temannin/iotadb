import socket

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 3306  # The port used by the server

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    while True:
        command = input("iotadb> ")
        if command == "clear":
            import os

            os.system("cls")
            continue
        if command == "exit":
            break
        if command == "crt":
            script = """
                CREATE TABLE Persons (
                    PersonID int,
                    LastName varchar(255),
                    FirstName varchar(255),
                    Address varchar(255),
                    City varchar(255)
                );
            """
            s.sendall(str.encode(script))
        else:
            s.sendall(str.encode(command))

        part = s.recv(4096)

    print("iotadb> Closing connection to server.")
    s.close()
