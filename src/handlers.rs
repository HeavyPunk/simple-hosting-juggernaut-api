use crate::{models::{TaskRequest, Task, TaskStatus}, configuration_provider};
use amiquip::{Connection, Exchange, Publish};
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

    let serialized_task=  serde_json::to_vec(&task).unwrap();

    let mut connection = Connection::insecure_open(&settings.rabbitmq_connection_string).unwrap();
    let channel = connection.open_channel(None).unwrap();
    let exchange = Exchange::direct(&channel);

    exchange.publish(Publish::new(&serialized_task, "default")).unwrap();
    connection.close().unwrap();

    Ok(warp::reply::json(&task_id))
}

pub async fn check_task_status(task_id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let settings = configuration_provider::get_config();
    let client = redis::Client::open(settings.redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
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
