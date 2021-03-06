#![feature(let_else)]

#[macro_use]
extern crate worker;

use tracker_core::prelude::*;
use uuid::Uuid;
use worker::*;

mod utils;

const KV_NAMESPACE: &str = "TRACKER_KV";

type Tracker = tracker_core::Tracker<Code>;

async fn generate_new_tracker<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    // generate a new id that will be the key in the kv store for the tracker
    // NOTE: this could potentially use v5 using some unique user data (e.g. email, name)
    let id = Uuid::new_v4();

    // get the optional url query params
    let url = req.url()?;
    let name = url.query_pairs().find(|(k, _)| k == "name");

    // create an empty tracker with an optional name
    // TODO: allow using Code or Class
    let tracker = match name {
        Some((_, name)) => Tracker::new(&name),
        _ => Tracker::default(),
    };

    // get access to kv store
    let Ok(kv) = ctx.kv(KV_NAMESPACE) else {
        return Response::error("Internal Server Error", 500);
    };

    // put the tracker into the kv store using the uuid created
    // tracker is serialized into json
    if kv.put(&id.to_string(), tracker)?.execute().await.is_ok() {
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

    // parse the json data from kv store into a Tracker
    let Some(tracker) = kv.get(&id.to_string()).json::<Tracker>().await? else {
        return Response::error("Not Found: UUID doesn't exist in KV", 404);
    };

    // return the Tracker as json, this function deserializes and then serializes since Response::from_json creates a response with the correct headers for json
    Response::from_json(&tracker)
}

async fn update_tracker<D>(mut req: Request, ctx: RouteContext<D>) -> Result<Response> {
    // ensure the param "uuid" is given and parses into UUID
    let Ok(id) = ctx.param("uuid").unwrap_or(&String::new()).parse::<Uuid>() else {
        return Response::error("Bad Request: UUID not provided", 400);
    };

    let Ok(updated) = req.json::<Tracker>().await else {
        return Response::error("Bad Request: JSON provided is invalid", 400);
    };

    // get access to kv store
    let Ok(kv) = ctx.kv(KV_NAMESPACE) else {
        return Response::error("Internal Server Error: could not connect to KV", 500);
    };

    // ensure that the UUID exists in the KV store
    if kv.get(&id.to_string()).text().await?.is_none() {
        return Response::error("Not Found: UUID doesn't exist in KV", 404);
    };

    // update the tracker in the KV store with the updated tracker
    if kv.put(&id.to_string(), updated)?.execute().await.is_ok() {
        return Response::ok(format!("Updated tracker: {}", id));
    }

    Response::error("Bad Request", 400)
}

async fn delete_tracker<D>(_req: Request, ctx: RouteContext<D>) -> Result<Response> {
    // ensure the param "uuid" is given and parses into UUID
    let Ok(id) = ctx.param("uuid").unwrap_or(&String::new()).parse::<Uuid>() else {
        return Response::error("Bad Request: UUID not provided", 400);
    };

    // get access to kv store
    let Ok(kv) = ctx.kv(KV_NAMESPACE) else {
        return Response::error("Internal Server Error: could not connect to KV", 500);
    };

    // delete the tracker with the uuid param
    if kv.delete(&id.to_string()).await.is_ok() {
        return Response::ok(format!("Deleted tracker: {}", id));
    }

    Response::error("Bad Request", 400)
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
        .post_async("/tracker/:uuid", update_tracker)
        .delete_async("/tracker/:uuid", delete_tracker)
        .run(req, env)
        .await
}
