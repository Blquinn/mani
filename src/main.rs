use std::str::FromStr;
use std::time::Duration;


use actix_http::body::Body;
use actix_web::{App, HttpResponse, HttpServer, client, error, post, web};
use futures::StreamExt;
use futures::prelude::stream::FuturesOrdered;
use log::*;

mod base64;
mod models;
mod settings;

use models::*;

// TODO: Make this configurable.
const BIND_ADDR: &str = "127.0.0.1:9999";
const HOST_WILDCARD: &str = "*";
// TODO: Make this configurable.
const MAX_SIZE: usize = 262_144; // max payload size is 256k

// TODO: Body encoding query string / multiple encodings.
#[post("/mani")]
async fn index(mut payload: web::Payload, cli: web::Data<client::Client>) -> Result<HttpResponse, error::Error> {

    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("Request body too large."));
        }
        body.extend_from_slice(&chunk);
    }

    let mani_req = serde_json::from_slice::<ManiRequestWrapper>(&body)?;

    // Validate request hosts.
    for r in mani_req.requests.iter() {
        let uri = http::Uri::from_str(r.url.as_str())
            .map_err(|_| { error::ErrorBadRequest(format!("Url {} is not a valid URI.", r.url)) })?;

        if let Some(host) = uri.host() {
            if !settings::SETTINGS.origins.contains(HOST_WILDCARD) && 
                !settings::SETTINGS.origins.contains(host) {
                return Err(error::ErrorBadRequest(format!("Url {} contains forbidden host.", r.url)))
            }
        } else {
            return Err(error::ErrorBadRequest(format!("Url {} doesn't contain valid host.", r.url)))
        }
    }

    let mut req_futs: FuturesOrdered<awc::SendClientRequest> = mani_req.requests.iter().map(|req| {
        let meth = http::Method::from_str(req.method.as_str()).unwrap();

        let mut request = cli.request(meth, req.url.clone());

        // TODO: Configurable timeouts.
        request = request.timeout(Duration::from_secs(10));

        for h in req.headers.iter() {
            request = request.header(h.key.as_str(), h.value.as_str());
        }

        let body = req.body.to_owned().map_or_else(
            ||{ Body::None }, 
            |b| { Body::Bytes(bytes::Bytes::from(b)) } );

        request.send_body(body)
    }).collect();

    /*
    let responses = req_futs.map(
        |res: Result<ClientResponse<_>, SendRequestError>| -> Box<dyn Future<Output = ManiResponse>> {
        match res {
            Ok(r) => {
                let headers = r.headers().iter().map(|(k, v)| {
                    ManiHeader{key: k.to_string(), value: v.to_str().unwrap().to_string()}
                }).collect();

                r.body().map(|body_res| {
                    match body_res {
                        Ok(res_body) => {
                            ManiResponse{
                                error: None,
                                response: Some(ManiResponseMessage{
                                    status_code: r.status().as_u16(),
                                    headers,
                                    body: Some(res_body.to_vec()),
                                })
                            }
                        },
                        Err(e) => {
                            ManiResponse{
                                error: Some(ManiResponseError{description: format!("{}", e)}),
                                response: None
                            }
                        }
                    }
                })
            },
            Err(e) => {
                futures::future::ok(ManiResponse{
                    error: Some(ManiResponseError{description: format!("{}", e)}),
                    response: None
                })
            }
        }
    }).into_future().await;
    */

    let mut mani_responses: Vec<ManiResponse> = Vec::new();
    // Reading of response bodies is done sequentially here.
    // There is definitely room for improvement (above).
    while let Some(res) = req_futs.next().await { 
        match res {
            Ok(mut r) => {
                let headers = r.headers().iter().map(|(k, v)| {
                    ManiHeader{key: k.to_string(), value: v.to_str().unwrap().to_string()}
                }).collect();

                match r.body().await {
                    Ok(res_body) => {
                        mani_responses.push(ManiResponse{
                            error: None,
                            response: Some(ManiResponseMessage{
                                status_code: r.status().as_u16(),
                                headers,
                                body: Some(res_body.to_vec()),
                            })
                        })
                    },
                    Err(e) => {
                        mani_responses.push(ManiResponse{
                            error: Some(ManiResponseError{description: format!("{}", e)}),
                            response: None
                        });
                    }
                }
            }
            Err(e) => {
                mani_responses.push(ManiResponse{
                    error: Some(ManiResponseError{description: format!("{}", e)}),
                    response: None
                });
            }
        }
    }

    Ok(HttpResponse::Ok().json(ManiResponseWrapper {
        responses: mani_responses
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Configured origins: {:?}", settings::SETTINGS.origins);

    HttpServer::new(|| {
        let cli = client::Client::default();

        App::new()
            .data(cli)
            .service(index)
    })
        .bind(BIND_ADDR)?
        .run()
        .await
}
