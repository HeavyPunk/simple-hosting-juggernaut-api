use warp::Filter;
use super::handlers;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    put_task()
}


fn put_task() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task/put")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::put_task)
}