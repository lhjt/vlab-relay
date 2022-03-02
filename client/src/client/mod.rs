use tonic::{
    codegen::InterceptedService,
    metadata::MetadataValue,
    transport::Channel,
    Request,
    Status,
};

use crate::{config::Configuration, relay::core::relay_service_client::RelayServiceClient};

/// The `gRPC` client to connect to the relay server.
pub(crate) struct RpcClient {
    config: Configuration,
}

impl RpcClient {
    async fn get_client(
        &self,
    ) -> RelayServiceClient<
        InterceptedService<Channel, impl FnOnce(Request<()>) -> Result<Request<()>, Status> + '_>,
    > {
        let config = &self.config;
        let uri = config.uri.clone();
        let uri = uri.unwrap();
        let channel = Channel::from_shared(uri).unwrap().connect().await.unwrap();

        RelayServiceClient::with_interceptor(channel, |mut req: Request<()>| {
            req.metadata_mut().insert(
                "token",
                MetadataValue::from_str(&format!("Bearer {}", config.token.clone().unwrap()))
                    .unwrap(),
            );
            Ok(req)
        })
    }

    pub(crate) async fn new(config: Configuration) -> Self { Self { config } }
}
