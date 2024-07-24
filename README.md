There is only one struct and service in the proto/message.

service- we impl chatservice  and take request from the client and respond back by adding from server side
client- we take input from the user and send it to the service and then if we get a response we display it

Run  ```cargo run --bin grpc-server ``` to run the server
Run  ```cargo run --bin grpc-client ``` to run the client
