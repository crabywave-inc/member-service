use anyhow::Result;
use futures::StreamExt;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use serde::Serialize;
use std::sync::Arc;

use crate::application::ports::messaging_ports::MessagingPort;

#[derive(Clone)]
pub struct PubSubMessaging {
    client: Arc<Client>,
    project_id: String,
}

impl PubSubMessaging {
    pub async fn new(project_id: String) -> Result<Self> {
        let config = ClientConfig::default().with_auth().await?;

        let client = Client::new(config).await?;

        Ok(PubSubMessaging {
            client: Arc::new(client),
            project_id,
        })
    }
}

impl MessagingPort for PubSubMessaging {
    async fn publish_message<T: Serialize>(&self, topic: String, message: T) -> anyhow::Result<()> {
        let t = format!("projects/{}/topics/{}", self.project_id, topic);

        let topic = self.client.topic(&t);

        if !topic.exists(None).await? {
            tracing::error!("Topic {} does not exist", t);
        }

        let message_str = serde_json::to_string(&message)?;

        let publisher = topic.new_publisher(None);

        let msg = PubsubMessage {
            data: message_str.into(),
            ordering_key: "order".into(),
            ..Default::default()
        };

        let awaiter = publisher.publish(msg).await;

        awaiter.get().await?;

        Ok(())
    }

    async fn subscribe<F, T, Fut>(
        &self,
        _topic: &str,
        group_id: &str,
        handler: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        T: serde::de::DeserializeOwned + Send + Sync + std::fmt::Debug + Clone + 'static,
    {
        let subscription_name = format!("projects/{}/subscriptions/{}", self.project_id, group_id);

        let subscription = self.client.subscription(&subscription_name);
        let mut stream = subscription.subscribe(None).await?;

        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                let msg: Vec<u8> = message.message.data.clone();

                let msg = match String::from_utf8(msg) {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!("Failed to parse message payload: {:?}", e);
                        continue;
                    }
                };

                let parsed_message: T = match serde_json::from_str(&msg) {
                    Ok(msg) => msg,
                    Err(e) => {
                        tracing::error!("Failed to parse message: {:?}", e);
                        continue;
                    }
                };

                if let Err(e) = handler(parsed_message).await {
                    tracing::error!("Failed to handle message: {:?}", e);
                }

                message.ack().await.unwrap();
            }
        });

        Ok(())
    }
}
