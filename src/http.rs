use std::ffi::OsStr;
use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use log::{debug, error, info};
use rouille::{extension_to_mime, Request, Response, router};
use rand::{distributions::Alphanumeric, Rng}; // 0.8

const NOT_FOUND_CONTENT:&str = include_str!("../404.html");
pub fn start_http(port:u32){

    rouille::start_server(format!("0.0.0.0:{port}"), move |request| {
        let log_ok = |req: &Request, resp: &Response, _elap: std::time::Duration| {
            info!("{} {} {} {}", request.remote_addr(), req.method(), req.raw_url(),req.header("Host").unwrap());
        };
        let log_err = |req: &Request, _elap: std::time::Duration| {
            error!("{} Handler panicked: {} {}", request.remote_addr(), req.method(), req.raw_url());
        };
        let queue_mutex:Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        rouille::log_custom(request, log_ok, log_err,  || {
            router!(request,
                (GET) (/) => {
                    debug!("{} {} {} {} redirecting to index.html ",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
                    resolve_get(request,"/index.html".to_string())
                },


                (POST) (/api/queue_image) => {
                    queue_image(request,queue_mutex);
                    Response::empty_204()
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

fn queue_image(request: &Request, queue_mutex:Arc<Mutex<Vec<String>>>){
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let name = format!("{0}-{1}",request.remote_addr(),s);
    let path = format!("./queue/{name}");
    let bytes = request.data().unwrap().bytes();
    let mut file:File = File::create(&path).unwrap();

    file.write_all(&[]).expect("could not clear file");
    for byte in bytes {
        file.write(&[byte.unwrap()]).expect("failed to write");
    }

    let mut queue = queue_mutex.lock().unwrap();
    debug!("{path} {queue:#?}");
    queue.push(name);
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