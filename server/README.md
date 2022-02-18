# server

The relay server that listens to requests from a relay client and forwards them to the `vl-client` for testing and submission of code on the student's VLab instance.

# Environment Variables

| Var        | Usage                                    | Default |
| ---------- | ---------------------------------------- | ------- |
| `RUST_LOG` | The level of logs to log to the console. | `INFO`  |

# Ports

| Port    | Usage            |
| ------- | ---------------- |
| `50051` | gRPC server      |
| `50052` | Websocket server |
