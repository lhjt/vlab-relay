# server

The relay server that listens to requests from a relay client and forwards them to the `vl-client` for testing and submission of code on the student's VLab instance.

## Communication

`client`s can connect to the server via a gRPC (or grpc-web) connection. The RPC service methods are defined in the [protocol definitions](/proto).

`vl-client`s connect and remain connected to the server via websockets.

# Self Hosting

It is your responsibility to ensure that you have the valid certificates set up when hosting this server, preferably behind some sort of proxy as there is no certificate configuration available for the gRPC server.

## Environment Variables

| Var        | Usage                                    | Default |
| ---------- | ---------------------------------------- | ------- |
| `RUST_LOG` | The level of logs to log to the console. | `INFO`  |

## Ports

| Port    | Usage            |
| ------- | ---------------- |
| `50051` | gRPC server      |
| `50052` | Websocket server |
