use crate::models::{TaskRequest, Task};
use rabbitmq_stream_client::{Environment, types::Message, error::ClientError};
use uuid::Uuid;

// TODO: научиться выстраивать адекватные пайплайны с обработкой всех ошибок
pub async fn put_task(body: TaskRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let task_id = Uuid::new_v4().to_string();

    let task = Task {
        id: task_id.clone(),
        kind: body.kind,
        context: body.context
    };

    let serialized_task=  serde_json::to_vec(&task).ok().expect("serialization failed");
    let environment = Environment::builder().build().await.ok().expect("environment building failed");
    let mut producer = environment.producer().name("tasks").build("stream").await.ok().expect("stream building error");
    producer
        .send_with_confirm(Message::builder().body(serialized_task).build())
        .await;
    producer.close().await.ok().expect("closing producer error");
    Ok(warp::reply::json(&task_id))
}
