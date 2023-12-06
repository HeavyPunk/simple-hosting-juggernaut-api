use crate::{models::{TaskRequest, Task, TaskStatus}, configuration_provider};
use rabbitmq_stream_client::{Environment, types::Message, error::ClientError};
use uuid::Uuid;
use redis::Commands;

// TODO: научиться выстраивать адекватные пайплайны с обработкой всех ошибок
pub async fn put_task(body: TaskRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let settings = configuration_provider::get_config();
    let task_id = Uuid::new_v4().to_string();

    let task = Task {
        id: task_id.clone(),
        kind: body.kind,
        context: body.context
    };

    let serialized_task=  serde_json::to_vec(&task).ok().expect("serialization failed");
    let environment = Environment::builder()
        .host(&settings.rabbitmq_host)
        .port(settings.rabbitmq_port.try_into().unwrap())
        .build().await.ok().expect("environment building failed");
    let mut producer = environment.producer().name("tasks").build("stream").await.ok().expect("stream building error");
    producer
        .send_with_confirm(Message::builder().body(serialized_task).build())
        .await;
    producer.close().await.ok().expect("closing producer error");
    Ok(warp::reply::json(&task_id))
}

pub async fn check_task_status(task_id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let settings = configuration_provider::get_config();
    let client = redis::Client::open(settings.redis_url).ok().expect("redis client constraction error");
    let mut con = client.get_connection().ok().expect("connection error");
    let task_status: Result<String, redis::RedisError> =  con.get(task_id.clone());
    if task_status.is_ok() {
        let response = TaskStatus {
            id: task_id.clone(),
            found: true,
            status: task_status.ok().expect("")
        };
        return Ok(warp::reply::json(&response));
    }
    else {
        let response = TaskStatus {
            id: task_id.clone(),
            found: false,
            status: "QUEUED".to_string()
        };
        return Ok(warp::reply::json(&response));
    }
}
