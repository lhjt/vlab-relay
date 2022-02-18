# VLAB Relay

A relay for UNSW students to allow local testing and submission of code for COMP courses without always having to SSH/VNC into VLab.

## Key Components

This project is made of the following 3 binaries:

1. [client](/client) - the CLI application run on the student's machine.
2. [vl-client](/vl-client) - the client application that remains running on VLab.
3. [server](/server) - the relay server that is used as an intermediary between the `client` and the `vl-client`.
