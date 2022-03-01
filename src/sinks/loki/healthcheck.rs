use super::config::LokiConfig;
use crate::http::HttpClient;

async fn fetch_status(
    endpoint: &str,
    config: &LokiConfig,
    client: &HttpClient,
) -> crate::Result<http::StatusCode> {
    let endpoint = config.endpoint.append_path(endpoint)?;

    let mut req = http::Request::get(endpoint.uri)
        .body(hyper::Body::empty())
        .unwrap();

    if let Some(auth) = &config.auth {
        auth.apply(&mut req);
    }

    Ok(client.send(req).await?.status())
}

pub(crate) async fn healthcheck(config: LokiConfig, client: HttpClient) -> crate::Result<()> {
    let endpoint = config.endpoint.append_path("ready")?;

    let mut req = http::Request::get(endpoint.uri)
        .body(hyper::Body::empty())
        .expect("Building request never fails.");

    if let Some(auth) = &config.auth {
        auth.apply(&mut req);
    }

    let res = client.send(req).await?;

    let status = match fetch_status("ready", &config, &client).await? {
        // Issue https://github.com/vectordotdev/vector/issues/6463
        http::StatusCode::NOT_FOUND => {
            debug!("Endpoint `/ready` not found. Retrying healthcheck with top level query.");
            fetch_status("", &config, &client).await?
        }
        status => status,
    };

    match status {
        http::StatusCode::OK => Ok(()),
        _ => Err(format!("A non-successful status returned: {}", res.status()).into()),
    }
}
