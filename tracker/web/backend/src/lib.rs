#![feature(let_else)]

#[macro_use]
extern crate worker;

use tracker_core::prelude::*;
use uuid::Uuid;
use worker::*;

mod utils;

const KV_NAMESPACE: &str = "TRACKER_KV";

async fn generate_new_tracker<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    // generate a new id that will be the key in the kv store for the tracker
    // NOTE: this could potentially use v5 using some unique user data (e.g. email, name)
    let id = Uuid::new_v4();

    // get the optional url query params
    let url = req.url()?;
    let name = url.query_pairs().find(|(k, _)| k == "name");

    // create an empty tracker with an optional name
    // TODO: allow using Code or Class
    let tracker: Tracker<Code> = match name {
        Some((_, name)) => Tracker::new(&name),
        _ => Tracker::default(),
    };

    // get access to kv store
    let Ok(kv) = ctx.kv(KV_NAMESPACE) else {
        return Response::error("Internal Server Error", 500);
    };

    // put the tracker into the kv store using the uuid created
    // TODO: replace format! with serialized Tracker
    if kv
        .put(&id.to_string(), format!("{:?}", tracker))?
        .execute()
        .await
        .is_ok()
    {
        // tracker was successfully put into kv store, return the uuid with status 201
        return Ok(Response::ok(id.to_string())?.with_status(201));
    }

    Response::error("Bad Request", 400)
}

async fn get_tracker<D>(_req: Request, ctx: RouteContext<D>) -> Result<Response> {
    // ensure the param "uuid" is given and parses into UUID
    let Ok(id) = ctx.param("uuid").unwrap_or(&String::new()).parse::<Uuid>() else {
        return Response::error("Bad Request: UUID not provided", 400);
    };

    // get access to kv store
    let Ok(kv) = ctx.kv(KV_NAMESPACE) else {
        return Response::error("Internal Server Error: could not connect to KV", 500);
    };

    // TODO: replace text() with json() using Tracker
    let Some(tracker) = kv.get(&id.to_string()).text().await? else {
        return Response::error("Not Found: UUID doesn't exist in KV", 404);
    };

    Response::ok(tracker)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    // get more helpful error messages written to the console in the case of a panic
    utils::set_panic_hook();

    let router = Router::new();
    router
        .post_async("/tracker/new", generate_new_tracker)
        .get_async("/tracker/:uuid", get_tracker)
        .run(req, env)
        .await
}
