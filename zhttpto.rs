//Altered by Mark Valeiras --- MDV3KT
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];



use std::io::*;
use std::io::File;
use std::io::net::ip::{SocketAddr};
use std::str;
use std::os;
use std::path::posix::Path;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut count: uint = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();

    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() 
    {
        // Spawn a task to handle the connection
        do spawn 
        {
            let mut stream = stream;
            
            match stream 
            {
                Some(ref mut s) => 
                {
                             match s.peer_name() 
                             {
                                Some(pn) => {unsafe{count += 1;} println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                                },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            
            let requestln = request_str.clone();
            println(requestln);

            let getpt = requestln.find_str("GET");
            let httppt = requestln.find_str("HTTP/1.1");
            
            let page: ~str = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 </body></html>\r\n";

            let mut response: ~str = match (getpt, httppt)
                {
                    (Some(i), Some(j)) if (j-i) >6 => serveFile(requestln.slice(i+5,j-1)),
                    (_,_) => page.clone().into_owned()
                
             };
         
            if str::eq(&response, &page)
               {
             unsafe{
                    response = response + format!("
                    <h2>Number of requests: {:u}</h2>
                    </body></html>\r\n", count);
                    } 

                }

            
            stream.write(response.as_bytes());
            println!("Connection terminates.");
        
        }
    }
}

fn serveFile(path: &str) -> ~str
{
    let file = Path::new(path);
    let mut opstr: ~str;

    if file.is_file() && !file.is_dir()
    {
        let mut msg_file_a = File::open(&file);
        let mut msg_bytes_a: ~[u8]= msg_file_a.read_to_end();
        let mut msg_a= str::from_utf8(msg_bytes_a);
        opstr = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>{:s}</title>
                 </head>
                 <body>
                 <h3> {:s} </h3>
                 </body></html>\r\n", path, msg_a)
    }   

    else
    {
        opstr = format!("HTTP/1.0 403 Forbidden\r\n");
    
    }
    opstr
    
}
