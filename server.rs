use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Error, Write, BufRead};
use std::net::{Ipv4Addr, TcpListener, SocketAddrV4, Shutdown};
use std::path::Path;

const CHUNK_SPLITS: usize = 1000;

fn main() -> Result<(), Error>{
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let server_socket = SocketAddrV4::new(localhost, 1337);

    println!("Starting up server on Port: {}...", server_socket.port());

    // Set up the listener for the first input
    let server_listener = TcpListener::bind(server_socket)?;

    // Wait for connection
    let (mut inc_stream, _) = server_listener.accept()?;
    let read_stream = inc_stream.try_clone()?;
    let mut reader = BufReader::new(read_stream);

    loop {
        // Check to see what type of action we must set up for
        // 1) Upload from client
        // 2) Download to client

        println!("Awaiting Client...");

        // Wait until input to read
        let mut action = String::new();
        while action == "" {
            reader.read_line(&mut action)?;
        }

        action = action.trim().to_string();
        println!("{}", action);

        // Upload route
        if "upload" == action {

            // Read incoming file name if not exiting
            let mut file_name = String::new();
            reader.read_line(&mut file_name)?;

            file_name = file_name.trim().to_string();

            // Read incoming file name
            file_name = "new".to_owned() + &file_name;

            // Create a buffer to receive the file size
            let mut file_size = [0; 8];
            inc_stream.read_exact(&mut file_size).unwrap();
            let file_size = u64::from_ne_bytes(file_size);

            println!("{}", file_size);

            // Read and output the client input
            let file = File::create(file_name)?;
            let mut writer = BufWriter::new(file);

            // io::copy(&mut reader, &mut writer)?;

            let mut bytes_remaining = file_size;
            let mut chunk = [0; CHUNK_SPLITS];

            while bytes_remaining > 0 {

                // If there are no more bytes to read then exit
                if bytes_remaining == 0 {
                    break;
                }

                // Get the amount of bytes that are left to be read
                // If there are enough bytes to fill chunk, fill it
                // Otherwise only fill with however many are left
                let bytes_in = min(bytes_remaining, chunk.len() as u64) as usize;

                // Read exactly bytes_in from stream
                reader.read_exact(&mut chunk[..bytes_in]).unwrap();
                // Write to stream
                writer.write_all(&chunk[..bytes_in]).unwrap();
                writer.flush().unwrap();

                // Reduce bytes remaining by bytes_in
                bytes_remaining -= bytes_in as u64;
            }

            println!("Finished\n")
        }

        // Download Route
        else if "get" == action {

            // Read incoming file name if not exiting
            let mut file_name = String::new();
            reader.read_line(&mut file_name)?;

            file_name = file_name.trim().to_string();

            // Check if desired file exists in system
            if Path::new(&file_name).exists() {

                // Confirm with client that file exists
                inc_stream.write(&[1]);

                // Open up the file for getting size
                let mut file = File::open(file_name)?;

                // Get length of upload file so send before writing the file
                let file_size = file.metadata().unwrap().len();
                println!("File Size: {}", file_size);
                let file_size_bytes = (file_size as u64).to_ne_bytes();
                inc_stream.write_all(&file_size_bytes).unwrap();
                inc_stream.flush().unwrap();

                // Make a buffer to hold the split data
                let mut chunk = [0; CHUNK_SPLITS];

                while let Ok(bytes) = file.read(&mut chunk) {

                    // If there are no more bytes to read then exit
                    if bytes == 0 {
                        break;
                    }

                    // Write to stream
                    // On final write, exclude the elements not read using
                    // Example: If there is one element left to write,
                    // we do not want the remaining 999 elements to be written.
                    inc_stream.write(&chunk[..bytes]).unwrap();
                }
                println!("Finished\n")
            } else { // File did not exist

                // Confirm with client that file does not exist
                inc_stream.write(&[0]);
                inc_stream.flush().unwrap();

                println!("File does not exist! \n");
            }

        }

        else if "exit" == action {
            println!("Shutting down the Server...");
            inc_stream.shutdown(Shutdown::Both)?;
            break;
        }

        // Error route
        else {
            println!("Client did not send proper request");
        }
    }

    Ok(())
}
