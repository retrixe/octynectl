use std::io::{Read, Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use uds_windows::UnixStream;

pub struct TokioCompatUnixStream {
    inner: UnixStream,
}

impl TokioCompatUnixStream {
    pub async fn connect(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let inner = tokio::task::block_in_place(|| UnixStream::connect(path))?;
        Ok(Self { inner })
    }
}

impl AsyncRead for TokioCompatUnixStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let inner = &mut self.inner;

        match tokio::task::block_in_place(|| inner.read(buf.initialize_unfilled())) {
            Ok(size) => {
                buf.advance(size);
                Poll::Ready(Ok(()))
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl AsyncWrite for TokioCompatUnixStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let inner = &mut self.inner;

        match tokio::task::block_in_place(|| inner.write(buf)) {
            Ok(size) => Poll::Ready(Ok(size)),
            Err(err) => Poll::Ready(Err(err)),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let inner = &mut self.inner;

        match tokio::task::block_in_place(|| inner.flush()) {
            Ok(_) => Poll::Ready(Ok(())),
            Err(err) => Poll::Ready(Err(err)),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
