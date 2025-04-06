### MODIFY THE CODE SUCH THAT IT CAN BE USED AS A LIBRARY 

### ADD UNINITS

### AMEND ALL INLINE TODOS

### FINISH NEATENING AND COMMENTING CODE

### DO SOME RESEARCH INTO EVENT DRIVEN USER INPUT SYSTEMS

### ADD SELECTION MENUS (ALREADY STARTED) IDEA SIMILAR TO THAT OF STARBOUNDS CHARACTER SELECTION

### ADD SSHING FOR SCARF TO ALLOW BETTER FILE ACCESS

1. ssh2 (Based on libssh2) - More Stable
ssh2 is a Rust wrapper around libssh2, making it a good choice for stability and performance.

"
[dependencies]
ssh2 = "0.9"
"

Here’s how to SSH into a server, execute a command, and capture output:


```
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect to the SSH server
    let tcp = TcpStream::connect("remote-server:22")?;
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    // Authenticate with password
    session.userauth_password("your-username", "your-password").unwrap();
    assert!(session.authenticated());

    // Execute a command
    let mut channel = session.channel_session().unwrap();
    channel.exec("ls -la").unwrap();

// Read output
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    println!("Output: {}", output);

    channel.wait_close().unwrap();
    Ok(())
}
```
✅ Works well, supports password & key authentication, and file transfers (SFTP).


Alternatively:


When Should You Use Async SSH?
✅ Use async (russh) if you:

Need to connect to multiple SSH servers at the same time.

Want to avoid blocking other tasks (e.g., user input, logging, web requests).

Are writing an async Rust application (e.g., an SSH-powered web API).

❌ Use sync (ssh2) if you:

Just need a simple SSH connection for quick commands.

Don’t care about performance when handling only one SSH session at a time.
