use distr_core::{ClusterInfo, Metrics, NodeInfo};
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

    let routes = get_cluster_info;
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

fn with_master(
    master: MasterNode,
) -> impl Filter<Extract = (MasterNode,), Error = Infallible> + Clone {
    warp::any().map(move || master.clone())
}

async fn handle_get_cluster_info(
    _token: String,
    master: MasterNode,
) -> Result<impl Reply, Rejection> {
    let nodes = master.nodes.lock().await;
    let cluster_info = ClusterInfo {
        nodes: nodes.values().cloned().collect(),
        master: NodeInfo::new(
            master.id,
            master.name.clone(),
            "master".to_string(),
            "".to_string(),
        ),
        cluster_name: "distributed_system".to_string(),
    };
    Ok(warp::reply::json(&cluster_info))
}
