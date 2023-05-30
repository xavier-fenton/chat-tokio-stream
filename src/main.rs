use tokio::{net::TcpListener, io::{ AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::broadcast};

#[tokio::main]

// use telnet localhost 8080 cmd to connect to server in a split terminal

// what is a 'turbofish' ? A way to give the rust compiler a specific value. Can search turbofish if I forget. Default::default() from the rust std

// at this point we aren't handling multiple clients independently. we are handling just one
async fn main() {
  let listener = TcpListener::bind("localhost:8080").await.unwrap();

  let (tx, _rx) = broadcast::channel(10);

  loop{
      let (mut socket, addr ) = listener.accept().await.unwrap();

      let tx = tx.clone();
      let mut rx = tx.subscribe();

      // tokio spawn is running concurrently and it is its own future and async block
      tokio::spawn(async move{
        let (reader, mut writer) = socket.split();

        let mut reader = BufReader::new(reader);
        let mut line = String::new();
        
        loop {
          tokio::select! {
            result = reader.read_line(&mut line) => {
              if result.unwrap() == 0 {
                break;
              }


              tx.send((line.clone(), addr)).unwrap();
              line.clear();
            }
            result = rx.recv() => {

              let (msg, other_addr) = result.unwrap();

              if addr != other_addr {
              writer.write_all(msg.as_bytes()).await.unwrap();
            }
            }
          }
        }});

  }
} 
