#[tokio::main]
async fn main() -> Result<(), eks::Error> {
    let client = eks::Client::from_env();
    let list_req = client.list_clusters();
    let list_resp = list_req.send().await?;
    let clusters = list_resp.clusters.unwrap();
    println!("Current EKS clusters: {:?}", clusters);
    for cluster in clusters {

        let describe_req = client.describe_cluster().name(cluster.to_owned());
        let describe_resp = describe_req.send().await?;
        println!("Cluster: {:?}", describe_resp);
        println!("******");

        let node_groups_req = client.list_nodegroups().cluster_name(cluster.to_owned());
        let node_groups_resp = node_groups_req.send().await?;
        println!("List of Nodegroups: {:?}", node_groups_resp);
    }
    Ok(())
}

/* TODO(joe): Also check for:
 *
 *  https://docs.aws.amazon.com/eks/latest/userguide/alb-ingress.html
 *  All private subnets tagged with:
 *  kubernetes.io/role/internal-elb = 1
 */
