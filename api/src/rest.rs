use distr_core::{ClusterInfo, NodeInfo, SystemMetrics};
use infrastructure::AuthService;
use services::MasterNode;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub async fn start_rest_api(master: MasterNode, auth_service: AuthService, port: u16) {
    let get_cluster_info = warp::path!("cluster" / "info")
        .and(warp::get())
        .and(warp::header::exact("Authorization", "Bearer valid_token"))
        .and(with_master(master.clone()))
        .and_then(handle_get_cluster_info);

    // Новий ендпоінт для системних метрик
    let get_system_metrics = warp::path!("system" / "metrics")
        .and(warp::get())
        .and(warp::header::exact("Authorization", "Bearer valid_token"))
        .and(with_master(master.clone()))
        .and_then(handle_get_system_metrics);

    let routes = get_cluster_info.or(get_system_metrics);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn handle_get_system_metrics(
    _token: String,
    master: MasterNode,
) -> Result<impl Reply, Rejection> {
    let system_metrics = master.system_metrics.lock().await;
    let metrics: Vec<_> = system_metrics.values().cloned().collect();
    Ok(warp::reply::json(&metrics))
}
