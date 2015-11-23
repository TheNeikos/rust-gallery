extern crate hyper;
extern crate url;

use std::path::{Path};
use std::fs::File;
use std::io::{Read, Write};

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;

use url::percent_encoding::percent_decode;

fn get_path_from_request(req: &Request) -> Result<String, ()> {
    match req.uri {
        RequestUri::AbsolutePath(ref st) => {
            String::from_utf8(percent_decode(st.as_bytes())).or(Err(()))
        }
        _ => Err(())
    }
}

fn send_404(mut res: Response) {
    *res.status_mut() = hyper::status::StatusCode::NotFound;
    res.send(b"Not found");
}

fn print_dir(path: &Path, res: Response) {
    let resp = format!("<!DOCTYPE html>
<html>
 <head>
     <title>Content of {title}</title>
     <style>{style}</style>
     <link href=\"data:image/png;base64,{favicon}\"
     rel=\"icon\" type=\"image/png\">
 </head>
 <body>
     <ul>
         <li><a href=\"..\">Parent Directory</a></li>
         {content}
     </ul>
 </body>
</html>", title=path.display() ,content={
        let dir = path.read_dir().unwrap();
        let mut v = Vec::new();
        for entry in dir {
            let dir = entry.unwrap();
            let name = dir.file_name().into_string().unwrap();
            v.push((format!("<li class=\"file\" data-type=\"{class}\"><a href=\"{link}{ending}\">{link}</a></li>", link = name,
               ending = {
                   if dir.file_type().unwrap().is_dir() { "/" } else { "" }
               },
               class = {
                   let ft = dir.file_type().unwrap();
                   if ft.is_dir() { format!("directory") } else {
                       let path = dir.path();
                       let name = path.extension();
                       format!("{}", match name {
                            Some(st) => st.to_str().unwrap(),
                            None => "unknown"
                       })
                   }
               }), dir.file_type().unwrap().is_dir()));
        }
        v.sort();

        v.into_iter().map(|x| x.0).collect::<Vec<String>>().join("")
    }, style = include_str!("./style.css"), favicon = include_str!("./rust-logo-32x32.base64"));
    res.send(resp.as_bytes());
}

fn send_file(path: &Path, res: Response) {
    let mut file = File::open(&path).unwrap();
    let mut res = res.start().unwrap();

    let mut buf = [0u8; 1024];
    while let Ok(x) = file.read(&mut buf) {
        if x == 0 { break; }
        res.write(&buf[0..x]);
    }

}

fn handle(req: Request, res: Response) {
    let path_str = {
        let path = get_path_from_request(&req);

        if let Ok(p) = path {
            format!(".{}", p)
        } else {
            return send_404(res);
        }
    };
    if path_str.find("..").is_some() {
        return send_404(res);
    }
    let mut path = Path::new(&path_str);
    let exists = path.exists();
    println!("{:?}", path);
    if exists {
        let is_dir = path.is_dir();
        if is_dir {
            print_dir(path, res);
        } else {
            send_file(path, res);
        }
    } else {
        send_404(res);
    }
}

fn main() {
    Server::http("0.0.0.0:3000").unwrap().handle(handle);
}
