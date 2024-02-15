use crate::Error;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use bpx_api_types::order::OrderInfo;
use ed25519_dalek::{Signature, Signer, SigningKey};
use futures::{Sink, SinkExt, Stream};
use pin_project::pin_project;
use serde::Deserialize;
use serde::Serialize;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message as WSMessage;
use url::Url;

const WSS_URL: &str = "wss://ws.backpack.exchange/stream";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub api_key: String,
    pub api_secret: String,
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
#[pin_project]
pub struct BpWebsocket {
    credential: Option<Credential>,
    #[pin]
    inner: WSStream,
}

impl BpWebsocket {
    async fn new_impl(credential: Option<Credential>) -> Result<Self, Error> {
        let (stream, _) = connect_async(Url::parse(WSS_URL).unwrap()).await?;
        Ok(Self {
            credential,
            inner: stream,
        })
    }

    pub async fn new_with_credential(credential: Credential) -> Result<Self, Error> {
        let mut client = Self::new_impl(Some(credential.clone())).await?;
        client.login().await?;
        Ok(client)
    }

    pub async fn login(&mut self) -> Result<(), Error> {
        let credential = self.credential.clone().unwrap();
        let api_secret = STANDARD
            .decode(credential.api_secret)?
            .try_into()
            .map_err(|_| Error::SecretKey)?;
        let signer = SigningKey::from_bytes(&api_secret);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis();
        let window = 5000;
        let signee = format!("instruction=subscribe&timestamp={timestamp}&window={window}");
        let signature: Signature = signer.sign(signee.as_bytes());
        let signature = STANDARD.encode(signature.to_bytes());

        // Subscribe to the depth stream.
        let cmd = Command::Subscribe {
            method: "SUBSCRIBE".into(),
            params: vec!["stream".into()],
            signature: vec![
                credential.api_key.clone(),
                signature,
                timestamp.to_string(),
                window.to_string(),
            ],
        };
        self.send(cmd).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub enum Command {
    Subscribe {
        method: String,
        params: Vec<String>,
        signature: Vec<String>,
    },
    Ping,
}

impl Sink<Command> for BpWebsocket {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        let this = self.project();
        let command = match &item {
            &Command::Ping => "ping".to_string(),
            command => serde_json::to_string(command)?,
        };
        Ok(this.inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_close(cx).map_err(|e| e.into())
    }
}

impl Stream for BpWebsocket {
    type Item = Result<Message, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => match parse_message(m) {
                Ok(m) => Poll::Ready(Some(Ok(m))),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Message {
    Pong,
    Order(OrderInfo),
}

fn parse_message(msg: WSMessage) -> Result<Message, Error> {
    match msg {
        WSMessage::Text(message) => match message.as_str() {
            "pong" => Ok(Message::Pong),
            others => match serde_json::from_str(others) {
                Ok(r) => Ok(r),
                Err(_) => Err(Error::ParseError(format!(
                    "Cannot deserialize message from '{}'",
                    others
                ))),
            },
        },
        WSMessage::Close(_) => Err(Error::WebsocketClosed),
        _ => unreachable!("Got unsupport websocket message type"),
    }
}
