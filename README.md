# rusty-file-transfer
Basic implementation of my own file transfer protocol between a client and server, both located on the same host machine.

# Requirements to Run
- Local Rust Installation
- Any Files for transferring

# How to Run

## 1 - Compile the rust files
Run the following two commands
```
rustc server.rs
rustc ftpclient.rs
```
A server.exe and ftpclient.exe file should exist afterwards.

## 2 - Launch the server
Run this only after compiling the server.rs file
`./server.exe`

## 3 - Run the Client
Run this only after compiling ftpclient.rs.
The proper port number is 1337 and the program will not progress unless that port is used.
`./ftpclient 1337`

## 4 - Use the Client
There are 3 different actions the client can take:
1. Get - Retrieve a file from the server
2. Upload - Upload a local file to the server
3. Exit - Disconnect and exit the program

For both get and upload, the following input is necessary:
`action file_name`
where action is either get or upload and the file_name is the file you wish to get or upload.
If the file_name is not local, then the client will prevent you from getting or uploading it.
The file must be on the system.

Exit does not require any other input.

# How it works
The server launches and immediately listens on the port 1337.
When the client launches it receives input from the user to connect to a specific port, 1337 is the only port that works.
The Upload aspect is implemented by:
1. Checking the file exists
2. Sending the file name and size to the Server
3. Uploading the file in 1kb chunks to the server
4. The server, knowing the size of the file, continuously reads from the socket until the final amount of bytes is read in.
5. The server writes this data to a new file based on the file name the user sent.
6. Both continue waiting for user input to continue.

The Get aspect is implemented by:
1. The client sending the file name to the server.
2. The server verifies that the file exists, if it does a 1 is sent to the client to prepare to read. Otherwise a 0 is sent as an error to the client.
3. The server continues, if the file exists, and sends the file size to the client.
4. Thus, the server writes to the client the entire file in chunks of 1kb.
5. The client reads in the file size so that it can read the exact amount of bytes from the server.
6. Once all the bytes have been read it will finish and exit the get statement.
