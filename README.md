# VLAB Relay

A relay for UNSW students to allow local testing and submission of code for COMP courses without always having to SSH/VNC into VLab.

> This isn't actually ready yet (it's WIP)

## Key Components

This project is made of the following 3 binaries:

1. [client](/client) - the CLI application run on the student's machine.
2. [runner](/runner) - the client application that remains running on VLab.
3. [server](/server) - the relay server that is used as an intermediary between the `client` and the `runner`.

## Usage

Prerequisites:

- An active relay server to connect to (you host it yourself).

1. Run the `runner` binary on VLab as a service (zellij, screen, up to you). It will automatically check your VLab username and attempt to connect and authenticate with the relay server.
2. On your local machine, run the client binary and login + connect to the relay service.

If successful, you should now be ready to run the relevant `autotest` and `give` commands on your local computer, which will then be relayed to VLab and executed accordingly.
