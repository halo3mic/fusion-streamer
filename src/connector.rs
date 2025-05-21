use tokio_tungstenite::{
    tungstenite::{Message, client::IntoClientRequest},
    connect_async,
};
use futures_util::{Stream, StreamExt, SinkExt};
use http::{uri::Uri, Request};
use tokio::sync::Mutex;
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use eyre::Result;
use url::Url;

pub struct Connector {
    endpoint: Url,
    auth: SecretString,
}

impl Connector {
    pub fn new(endpoint: Url, auth: impl Into<SecretString>) -> Self {
        Self { endpoint, auth: auth.into() }
    }

    pub async fn open(self) -> Result<impl Stream<Item = Option<Result<String>>>> {
        let req = self.build_request_with_auth()?;
        let (ws, _resp) = connect_async(req).await?;
        let (write, read) = ws.split();
        let write = Arc::new(Mutex::new(write));

        let text_stream = read.then({
            let write = write.clone();
            move |msg| {
                let write = write.clone();
                async move {
                    match msg {
                        Ok(Message::Ping(p)) => {
                            let _ = write.lock().await.send(Message::Pong(p)).await;
                            None
                        }
                        Ok(Message::Text(t)) => Some(Ok(t)),
                        Ok(Message::Close(_)) => None,
                        Err(e) => Some(Err(e.into())),
                        _ => None,
                    }
                }
            }
        });

        Ok(text_stream)
    }

    fn build_request_with_auth(&self) -> Result<Request<()>> {
        let mut req = self.endpoint
            .as_str()
            .parse::<Uri>()?
            .into_client_request()?;
        req.headers_mut().insert(
            "Authorization",
            format!("Bearer {}", self.auth.expose_secret()).parse()?,
        );
        Ok(req)
    }

}
