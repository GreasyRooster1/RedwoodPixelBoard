use std::ffi::OsStr;
use std::fmt::format;
use std::fs::File;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use log::{debug, error, info};
use rouille::{extension_to_mime, Request, Response, router};

const NOT_FOUND_CONTENT:&str = include_str!("../404.html");
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
                    resolve_get(request,"/index.html".to_string())
                },

                _ => {
                    if request.method() == "GET" {
                        return resolve_get(request,request.url())
                    }
                    not_found_response()
                }
            )
        })
    });//,cert,pkey).unwrap().run();
}

fn resolve_get(request: &Request,req_path:String) ->Response{
    debug!("{} {} {} {} requested file read",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
    let path = match get_path(req_path){
        Ok(p) => p,
        Err(_) => {
            return not_found_response();
        }
    };
    let binding = path.clone();
    let extension = Path::new(&binding).extension().and_then(OsStr::to_str).unwrap();
    let file = match File::open(path) {
        Ok(f)=>f,
        Err(_) =>{
            return not_found_response();
        }
    };
    Response::from_file(extension_to_mime(extension),file)
}

fn get_path(uri:String)->Result<String,String>{
    let path = PathBuf::from_str(format!("./data/{uri}").as_str()).unwrap();
    if path.components().any(|x| x == Component::ParentDir) {
        return Err("directory traversal".to_string());
    }
    Ok(path.as_path().to_str().unwrap().to_string())
}

fn not_found_response()->Response{
    Response::from_data("text/html",NOT_FOUND_CONTENT)
}