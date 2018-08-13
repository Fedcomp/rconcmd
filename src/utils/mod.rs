#![allow(deprecated)]

use std::thread;
use std::io;
use futures::sync::mpsc::{unbounded, SendError, UnboundedReceiver};
use tokio::prelude::stream::iter;
use futures::Sink;
use futures::Stream;
use futures::Future;
use std::io::BufRead;

#[derive(Debug)]
enum Error {
    Stdin(io::Error),
    Channel(SendError<String>),
}

pub fn spawn_stdin_stream_unbounded() -> UnboundedReceiver<String> {
    let (channel_sink, channel_stream) = unbounded();
    let stdin_sink = channel_sink.sink_map_err(Error::Channel);

    thread::spawn(move || {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        iter(stdin_lock.lines())
            .map_err(Error::Stdin)
            .forward(stdin_sink)
            .wait()
            .unwrap();
    });

    channel_stream
}
