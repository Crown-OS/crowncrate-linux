# Communication Module
**This module handles the communication between devices using a custom lightweight TCP powered protocol that keeps the connection alive till the device gets disconnected similar to websockets**
`

Each device is handled by a thread using a one line json.

Parsing schema `Message.rs`
Message: {
- method: -> Methods can be one of the operations listed below
- client: -> IP Address of the client
- body:   -> Contains the data required for the request
}

Method: 
- CLIPBOARD
- MEDIA
- OPEN
- OTPSYNC
- MONITOR
