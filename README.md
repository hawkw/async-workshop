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
  This username will identify that client to other peers connected to that server.
+ When the server receives a username from a client, it **broadcasts** a message
  to every other peer that a new client has joined the server, including that
  client's username and IP address.
+ After a client has joined a server, every subsequent message recieved by the
  server from that client is considered a **chat message**. The server
  broadcasts these messages to every other client.
+ A server implements a single chat channel. Each message sent to the server by
  a peer is broadcast to all other connected peers.
+ When a client disconnects, the server broadcasts a message to every other
  client indicating this.

## exercises

### workflow

I've included solutions for both exercises in the repository. In each exercise,
several modules contain unimplemented code which you'll be writing over the
course of the workshop. There are `#[path="..."]` attributes which you can
comment out to switch from the solution to your own code. This means that you
can run the completed version of the exercise, or in some cases, implement the
exercise one example at a time.

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
cargo -p client run -- <USERNAME>
```

or

```bash
cargo -p client run  -- <USERNAME> --server <SERVER_IP>:<SERVER_PORT>
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
Again, I've provided a skeleton for you to build on top of. The server is split
into a couple of files, and contains a few functions that are unimplemented.
Your job is to implement these.

Like the client, you can run the server using
```bash
cargo server
```

### bonus: making it fancier

If you finished early and enjoyed working on implementing the chat server, here
are some  additional features that it might be fun to add:

+ **direct messages**: Right now, all messages are public — they're broadcast
  to the whole channel. It would be neat to add support for sending direct
  messages to a specific user that aren't seen by the rest of the channel.
  We could add a command like `/tell <USER> <MESSAGE>`. When the server receives
  a message like this, it would forward the message only to the specified user,
  rather than broadcasting it to everyone.

  *Hint*: The first step will probably involve adding code to the server to
  allow looking up a specific peer by username. What information are we already
  storing that could help with this?
+ **channels**: Other chat systems — like IRC and Slack — have a concept of
  _channels_ for discussing certain topics or separating users into groups. We
  could add channels to our chat protocol as well.

  *Hint*: How would you adjust our protocol to give the server the additional
  information it needs to pass messages to a specific channel? Would we need to
  change the way a client connects to the server?
+ **messages with newlines**: Right now, our chat protocol is
  _newline-delimited_: this means that messages are broken into frames based on
  the newline character.

  *Hint*: If we wanted to support newlines in messages, we'd have
  to change both our client and server to support [some other method][len] of
  delimiting frames...

Note that I _don't_ have working implementations prepared for all these
features. If you're interested in working on them, I'm happy to provide
guidance, but I don't have finished solutions you can compare your work to.

[len]: https://docs.rs/tokio/0.2.0-alpha.4/tokio/codec/length_delimited/index.html

### further work

If you *really* enjoyed working on this exercise and are feeling particularly
adventurous, here are some ideas for bigger projects. These are almost certainly
out of scope for a three-hour workshop, but might make fun projects to hack on
for a few days.

#### implementing IRC

If you're feeling adventurous, we could try to turn our little chat system into a
full-on implementation of the IRC protocol, as described in [RFC 1459][irc].

Although IRC is relatively simple, implementing a real protocol is going to take
a bit more work than our toy protocol. An IRC implementation could be a good
side project to spend a few days on.

[irc]: https://tools.ietf.org/html/rfc1459

#### designing your own protocol

Protocol design is almost always driven by constraints. The most important
constraint that informed the design of our toy chat protocol was that it should
be possible to implement in the time allotted for this workshop. :) It could be
interesting to think about how you might design a new messaging protocol from
scratch, based on different constraints or requirements. Here are some questions
to consider:

- What would you need to do if you wanted to allow users to send images, or
  other kinds of files?
- How would you implement add a persistent notion of identity for your system?
- What would you change if you wanted to support keeping the chat history to
  reference in the future?
- What if we wanted to run our chat system on mobile, where network access can
  be spotty? How would we cope with network partitions or degraded performance?

Many of these are hard problems without simple solutions, but thinking about
them — and maybe solving one or two — could be both fun and a valuable learning
experience.
