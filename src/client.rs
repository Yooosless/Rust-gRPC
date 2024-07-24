pub mod pb {
    tonic::include_proto!("message");
}

use pb::{chat_service_client::ChatServiceClient, ChatMessage};
use tokio::io::{self, AsyncBufReadExt};
use tokio::io::{AsyncBufRead, BufReader};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::Request;

pub async fn input() -> String {
    println!("input something...");
    let mut input = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader
        .read_line(&mut input)
        .await
        .expect("failed to read input");
    input.trim().to_string()
}

async fn chat(client: &mut ChatServiceClient<Channel>) {
    let (tx, rx) = mpsc::channel(1);
    let in_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        loop {
            {
                let user_msg = input().await;
                if user_msg.eq_ignore_ascii_case("exit") {
                    break;
                } else {
                    let msg = ChatMessage {
                        message: user_msg,
                        from: String::from("Client"),
                    };
                    if tx.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    let response = client.chat_message_streaming(Request::new(in_stream)).await.unwrap();

    let mut resp_stream = response.into_inner();
    while let Some(receive) = resp_stream.next().await {
        let item = receive.unwrap();
        println!("Received {:?} from {:?}", item.message, item.from);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    println!("client started");
    chat(&mut client).await;
    Ok(())
}
