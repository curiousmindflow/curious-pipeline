use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

use crate::Error;

pub struct Socket<T>
where
    T: AsyncRead + AsyncWrite + Send + Unpin,
{
    socket: BufReader<T>,
}

impl<T> Socket<T>
where
    T: AsyncRead + AsyncWrite + Send + Unpin,
{
    pub fn new(socket: T) -> Socket<T> {
        Socket {
            socket: BufReader::new(socket),
        }
    }

    pub async fn write<M>(&mut self, message: M) -> Result<(), Error>
    where
        M: Serialize,
    {
        let message = serde_json::to_string(&message)?;
        let mut buffer = BytesMut::with_capacity(4096);
        buffer.put(message.as_bytes());

        self.socket.write_buf(&mut buffer).await.unwrap();
        self.socket.flush().await.unwrap();

        Ok(())
    }

    pub async fn read<M>(&mut self) -> Result<M, Error>
    where
        for<'de> M: Deserialize<'de>,
    {
        let mut buffer = String::new();

        self.socket.read_line(&mut buffer).await.unwrap();

        let message = serde_json::from_str::<M>(&buffer).unwrap();

        Ok(message)
    }
}
