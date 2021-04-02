use std::convert::Infallible;

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn create() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn update() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("<h1>hello</h1>"))
}

pub async fn fetch(id: String) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(format!("<h1>{}</h1>", id)))
}
