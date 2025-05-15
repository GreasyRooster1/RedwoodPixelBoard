use std::fmt::format;
use log::{debug, error, info};
use rouille::{Request, Response, router};

pub fn start_http(port:u32){
    rouille::start_server(format!("0.0.0.0:{port}"), move |request| {
        let log_ok = |req: &Request, resp: &Response, _elap: std::time::Duration| {
            info!("{} {} {} {}", request.remote_addr(), req.method(), req.raw_url(),req.header("Host").unwrap());
        };
        let log_err = |req: &Request, _elap: std::time::Duration| {
            error!("{} Handler panicked: {} {}", request.remote_addr(), req.method(), req.raw_url());
        };
        rouille::log_custom(request, log_ok, log_err,  || {
            router!(request,
                (GET) (/) => {
                    debug!("{} {} {} {} redirecting to index.html ",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
                    Response::
                },

                (GET) (/stats) => {
                    panic!("Not implemented yet")
                },

                _ => rouille::Response::empty_404()
            )
        })
    });//,cert,pkey).unwrap().run();
}