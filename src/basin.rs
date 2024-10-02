use s2::{
    client::BasinClient,
    service_error::{
        CreateStreamError, DeleteStreamError, GetStreamConfigError, ListStreamsError,
        ReconfigureStreamError, ServiceError,
    },
    types::{
        CreateStreamRequest, DeleteStreamRequest, ListStreamsRequest, ListStreamsResponse,
        ReconfigureStreamRequest, StreamConfig,
    },
};

pub struct BasinService {
    client: BasinClient,
}

#[derive(Debug, thiserror::Error)]
pub enum BasinServiceError {
    #[error("Failed to list streams: {0}")]
    ListStreams(#[from] ServiceError<ListStreamsError>),

    #[error("Failed to create stream")]
    CreateStream(#[from] ServiceError<CreateStreamError>),

    #[error("Failed to delete stream")]
    DeleteStream(#[from] ServiceError<DeleteStreamError>),

    #[error("Failed to get stream config")]
    GetStreamConfig(#[from] ServiceError<GetStreamConfigError>),

    #[error("Failed to reconfigure stream")]
    ReconfigureStream(#[from] ServiceError<ReconfigureStreamError>),
}

impl BasinService {
    pub fn new(client: BasinClient) -> Self {
        Self { client }
    }

    pub async fn list_streams(
        &self,
        prefix: String,
        start_after: String,
        limit: usize,
    ) -> Result<Vec<String>, BasinServiceError> {
        let list_streams_req = ListStreamsRequest::new()
            .with_prefix(prefix)
            .with_start_after(start_after)
            .with_limit(limit);

        let ListStreamsResponse { streams, .. } =
            self.client.list_streams(list_streams_req).await?;

        Ok(streams)
    }

    pub async fn create_stream(
        &self,
        stream: String,
        config: Option<StreamConfig>,
    ) -> Result<(), BasinServiceError> {
        let mut create_stream_req = CreateStreamRequest::new(stream);

        if let Some(config) = config {
            create_stream_req = create_stream_req.with_config(config);
        };

        self.client.create_stream(create_stream_req).await?;
        Ok(())
    }

    pub async fn delete_stream(&self, stream: String) -> Result<(), BasinServiceError> {
        self.client
            .delete_stream(DeleteStreamRequest::new(stream))
            .await?;
        Ok(())
    }

    pub async fn get_stream_config(
        &self,
        stream: String,
    ) -> Result<StreamConfig, BasinServiceError> {
        Ok(self.client.get_stream_config(stream).await?)
    }

    pub async fn reconfigure_stream(
        &self,
        stream: String,
        config: StreamConfig,
        mask: Vec<String>,
    ) -> Result<(), BasinServiceError> {
        let reconfigure_stream_req = ReconfigureStreamRequest::new(stream)
            .with_config(config)
            .with_mask(mask);

        self.client
            .reconfigure_stream(reconfigure_stream_req)
            .await?;
        Ok(())
    }
}
