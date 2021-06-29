use crate::config::GraphOpts;
use crate::result::Result;
use k8s_openapi::api::core::v1 as api;
use kube::api::{Api, ListParams, ResourceExt};
use kube::Client;
use crate::model::{Graph, Application};
use crate::dot;
use std::fs::File;

pub async fn graph(opts: &GraphOpts) -> Result<()> {
    let mut graph = Graph::new();
    
    let client = Client::try_default().await?;

    let cms: Api<api::ConfigMap> = Api::all(client);

    let lp = ListParams::default();

    for cm in cms.list(&lp).await? {
        if matches(opts, &cm) {
            println!("found ConfigMap: {}/{}", cm.namespace().unwrap_or_default(), cm.name());
            graph.applications.push(Application {
                namespace: cm.namespace().unwrap_or_default(),
                name: cm.name(),
                app_id: cm.data.get("APPLICATION_ID").map(|s| s.to_string()).unwrap(),
                consumes: cm.data.get("INPUT_TOPIC").map(|s| s.to_string()),
                produces: cm.data.get("OUTPUT_TOPIC").map(|s| s.to_string()),
            });
        }
    }

    if let Some(ref out) = opts.out {
        let mut f = File::create(out)?;
        dot::dot(&mut f, graph)?;
    } else {
        dot::dot(&mut std::io::stdout(), graph)?;
    }

    Ok(())
}

fn matches(_opts: &GraphOpts, cm: &api::ConfigMap) -> bool {
    let data = &cm.data;

    return data.contains_key("APPLICATION_ID")
        && (data.contains_key("INPUT_TOPIC")
        || data.contains_key("OUTPUT_TOPIC"));
}
