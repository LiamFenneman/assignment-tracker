#[macro_use]
extern crate worker;

use worker::*;

mod utils;

const KV_NAMESPACE: &str = "TRACKER_KV";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    // get more helpful error messages written to the console in the case of a panic
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/tracker/:id", |_, ctx| {
            let s = String::new();
            match ctx.param("id").unwrap_or(&s).parse::<u32>() {
                Ok(i) => Response::ok(format!("Tracker {}!", i)),
                _ => Response::error("Bad Request", 400),
            }
        })
        .get_async("/kv/:key", |_, ctx| async move {
            let kv = ctx.kv(KV_NAMESPACE)?;
            if ctx.param("key").is_none() {
                return Response::error("Bad Request", 400);
            };
            let key = ctx.param("key").expect("None checked above");

            let txt = kv.get(key).text().await;
            match txt {
                Ok(opt) if opt.is_some() => Response::ok(opt.unwrap()),
                _ => Response::error("Bad Request", 400),
            }
        })
        .post_async("/kv/:key", |mut req, ctx| async move {
            let kv = ctx.kv(KV_NAMESPACE)?;
            if let Some(key) = ctx.param("key") {
                if kv.put(key, req.text().await?)?.execute().await.is_ok() {
                    return Ok(Response::ok("Updated KV")?.with_status(201));
                }
            };

            Response::error("Bad Request", 400)
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
