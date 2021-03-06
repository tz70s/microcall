// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use futures;
use futures::{Future, Stream};
use futures::future::{FutureResult, ok};
use hyper::{Body, Client, Error, Get, Post, Request, Response, StatusCode};
use hyper;
use hyper::server::{Http, Service};
use tokio_core;
use tokio_core::reactor::Handle;
use uuid::Uuid;

use action;

#[derive(Debug)]
pub struct FunctionMeta {
    pub name: String,
}

struct Controller {
    client: hyper::Client<hyper::client::HttpConnector, Body>,
    handle: Handle,
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
}

impl Controller {
    fn new(
        client: hyper::Client<hyper::client::HttpConnector, Body>,
        handle: Handle,
        function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
    ) -> Self {
        Controller {
            client,
            handle,
            function_metas,
        }
    }
}

#[inline]
fn get_root() -> Box<FutureResult<Response, Error>> {
    Box::new(ok(Response::new()
        .with_status(StatusCode::Ok)
        .with_body("Hello World!")))
}

fn get_all(
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
) -> Box<FutureResult<Response, Error>> {
    let function_meta_mut_ref = function_metas.read().unwrap();
    let mut format_string = String::new();
    for (id, meta) in function_meta_mut_ref.iter() {
        format_string += format!("{:?}, {:?}\n", *id, *meta).as_str();
    }
    Box::new(ok(Response::new()
        .with_status(StatusCode::Ok)
        .with_body(format_string)))
}

fn bad_request() -> Box<FutureResult<Response, Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::NotFound);
    Box::new(ok(response))
}

impl Service for Controller {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            // Root path, say welcome message, remains for performance test.
            (&Get, "/") => get_root(),
            // Get all existed function instances.
            (&Get, "/all") => get_all(self.function_metas.clone()),
            // Deploy a function instance.
            (&Post, "/deploy") => {
                action::post_deploy(req, self.function_metas.clone(), self.client.clone())
            }
            // Invoke an endpoint/function
            _ if &req.path()[0..10] == "/endpoint/" => action::get_endpoint(req, &self.client),
            _ if &req.path()[0..14] == "/uds-endpoint/" => {
                action::get_uds_endpoint(req, self.handle.clone())
            }
            // 400
            _ => bad_request(),
        }
    }
}

pub fn launch(addr_str: &str) {
    let addr = addr_str.parse().unwrap();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client_handle = core.handle();
    let uds_handle = core.handle();
    let client = Client::configure().build(&client_handle);
    let function_metas = RwLock::new(HashMap::new());
    let arc = Arc::new(function_metas);

    let serve = Http::new()
        .serve_addr_handle(&addr, &handle, move || {
            Ok(Controller::new(
                client.clone(),
                uds_handle.clone(),
                arc.clone(),
            ))
        })
        .unwrap();
    info!("Listening on http://{}", serve.incoming_ref().local_addr());

    let h2 = handle.clone();
    handle.spawn(
        serve
            .for_each(move |conn| {
                h2.clone().spawn(conn.map(|_| ()).map_err(|_| ()));
                Ok(())
            })
            .map_err(|_| ()),
    );

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
