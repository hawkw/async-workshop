# exercise: implementing a chat server

## introduction

In this excercise, we're implementing a client and server for a relatively
simple text-based multi-user chat protocol, like a stripped-down version of IRC
or Slack.. We'll be using `tokio` and async/await.

## the protocol

Here's an overview of the protocol we're implementing:

+ The protocol consists of two kinds of peer: **clients** and **servers**. A
  server acts as a broker that broadcasts messages from one client to the other
  clients in the chat.
+ Clients and servers communicate over **TCP**. Messages are encoded in UTF-8
  and delimited by newline (`\n`) characters.
+ A client initiates a TCP connection to a server and sends a message consisting
  of a string representing the client's **username**, followed by a newline.
  This username will identify that client to other peers in the channel.
+ When the server receives a username from a client, it **broadcasts** to every
  other peer that a new client has joined the channel.
+ After a client has joined a channel, every subsequent message recieved by the
  server from that client is considered a **chat message**. The server
  broadcasts these messages to every other client.
+ When a client disconnects, the server broadcasts a message to every other
  client indicating this.

## exercises

### part 1: implementing the client

To dip our toes into async networking with tokio, we'll start by implementing a
simple command-line client for the chat protocol. The client will initiate a
connection to a server, read lines from standard in to send to the server, and
print messages received from the server.

Unless something goes horribly wrong with the conference center WiFi, I'll be
running an implementation of the  server for this protocol, so you can test out
your client against it. When you finish implementing your client, you can join
the server and chat with other participants in the workshop.

The file `src/bin/client.rs` contains a skeleton of the client implementation.
It handles parsing command-line arguments, and has some helper functions to get
started, as well as some comments.

You can run the client with

```bash
cargo run --bin client -- <USERNAME>
```

or

```bash
cargo run --bin client -- <USERNAME> --server <SERVER_IP>:<SERVER_PORT>
```

to connect to the server running on a specific address.

I've also included a `.cargo/config` with a `client` alias, so you can just say

```bash
cargo client <args...>
```

instead.

Good luck!

### part 2: implementing the server

Once you've gotten the client working, we'll move on to implementing the server.
Again, I've provided a skeleton for you to build on top of.

TODO(ELIZA) FINISH THIS PART

### part 3: getting fancier

If you enjoyed working on implementing the chat server, there are some
additional features that it might be fun to add:

+ **direct messages**: Right now, all messages are public — they're broadcast
  to the whole channel. It would be neat to add support for sending direct
  messages to a specific user that aren't seen by the rest of the channel.

  We could add a command like `/tell <USER> <MESSAGE>`. When the server receives
  a message like this, it would forward the message only to the specified user,
  rather than broadcasting it to everyone.
+ **channels**: Other chat systems — like IRC and Slack — have a concept of
  _channels_ for discussing certain topics or separating users into groups. We
  could add channels to our chat protocol as well.
+ **messages with newlines**: Right now, our chat protocol is
  _newline-delimited_: this means that messages are broken into frames based on
  the newline character. If we wanted to support newlines in messages, we'd have
  to change both our client and server to support [some other method][len] of
  delimiting frames...

### further work: implementing IRC

If you're feeling adventurous, we could try to turn our little chat system into a
full-on implementation of the IRC protocol, as described in [RFC 1459][irc]. This
would let our client connect to any IRC existing server, or our server accept
connections from any IRC client.

Although IRC is relatively simple, implementing a real protocol is going to take
a bit more work than our toy protocol. An IRC implementation could be a good
side project to spend a few days on.

[len]: https://docs.rs/tokio/0.2.0-alpha.4/tokio/codec/length_delimited/index.html
[irc]: https://tools.ietf.org/html/rfc1459

## workflow & debugging

TODO(ELIZA) FINISH THIS PART

## getting to know the crates

TODO(ELIZA) FINISH THIS PART
