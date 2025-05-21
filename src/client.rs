use futures_util::{StreamExt, stream::BoxStream};
use secrecy::SecretString;
use eyre::Result;
use url::Url;

use crate::events::{FusionEvent, UnknownEvent};
use crate::connector::Connector;


const DEFAULT_VERSION: &str = "v2.0";

pub struct FusionClient<NetworkId = u128> {
    connector: Connector,
    network_id: NetworkId,
}

impl<NetworkId> FusionClient<NetworkId> 
    where NetworkId: ToString + Clone + 'static
{

    pub fn new(
        endpoint: Url,
        network_id: NetworkId,
        auth: impl Into<SecretString>
    ) -> Self {
        Self::with_version(endpoint, DEFAULT_VERSION, network_id, auth)
    }

    pub fn with_version(
        endpoint: Url,
        version: &str,
        network_id: NetworkId,
        auth: impl Into<SecretString>
    ) -> Self {
        let network_endpoint = Self::build_network_endpoint(
            endpoint, 
            version, 
            network_id.clone(),
        );
        let connector = Connector::new(network_endpoint, auth);
        Self { connector, network_id }
    }

    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    pub async fn into_stream(self) -> Result<BoxStream<'static, Result<FusionEvent>>> {
        let stream = self.connector.open().await?;
        Ok(stream.filter_map(Self::parse_msg).boxed())
    }

    async fn parse_msg(msg: Option<Result<String>>) -> Option<Result<FusionEvent>> {
        match msg {
            Some(Ok(msg)) => {
                match serde_json::from_str(&msg) {
                    Ok(event) => {
                        tracing::debug!("Received Fusion event: {event:?}");
                        Some(Ok(event))
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse Fusion msg '{msg}': {e}");
                        match serde_json::from_str::<UnknownEvent>(&msg) {
                            Ok(event) => {
                                tracing::debug!("Received unknown Fusion event: {event:?}");
                                Some(Ok(FusionEvent::Unknown(event)))
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse unknown Fusion event: {e}");
                                Some(Err(e.into()))
                            }
                        }
                    }
                }
            }
            Some(Err(e)) => {
                tracing::error!("Error in Fusion stream: {e}");
                Some(Err(e))
            },
            None => None,
        }
    }

    fn build_network_endpoint(
        mut endpoint: Url,
        version: &str,
        network_id: NetworkId,
    ) -> Url {
        endpoint.path_segments_mut()
            .expect("base URL cannot be opaque")
            .pop_if_empty()
            .extend(&[version, &network_id.to_string()]);
        endpoint
    }
}
