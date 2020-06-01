#[derive(Debug)]
struct Request {
    method: String,
    host: String,
    user_agent: String
}

impl Request {

    fn parse_request(raw_request: &[u8]) -> Request{

        let request_data = String::from(String::from_utf8_lossy(&raw_request[..]).into_owned());
        
        let mut parameters: std::vec::Vec<&str> = request_data.split("\r\n").collect();
        
        // To find a better way to do it
        let request_method_vec: &std::vec::Vec<&str> = &parameters[0].split(" ").collect();
        let host_vec: &std::vec::Vec<&str> = &parameters[1].split(": ").collect();
        let user_agent_vec: &std::vec::Vec<&str> = &parameters[2].split(": ").collect();

        let request_method: String = String::from(request_method_vec[0]);
        let host: String = String::from(host_vec[1]);
        let user_agent: String = String::from(user_agent_vec[1]);

        return Request{
            method: request_method,
            host: host,
            user_agent: user_agent
        }

    }
}

struct Response<'a> {
    status_code: &'a str,
    http_version: &'a str
}

impl<'a> Response<'a> {
    
    pub fn build_dumb_response() -> Self{
        let mut new_response = Self{
            status_code: "200 OK",
            http_version: "HTTP/1.1",
        };
        
        return new_response
    }

    pub fn to_str(self) -> std::string::String {
        let response_array = format!("{} {}", self.status_code, self.http_version);

        return response_array;
    }
}
impl<'a> std::fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.http_version, self.status_code)
    }
}

mod server {
    use std::net;
    use std::io::Read;
    use std::io::Write;
    use Request;
    use Response;

    const SERVER_ADDR: &str = "127.0.0.1:8000";
      
    pub fn start() {
        let listener = net::TcpListener::bind(&SERVER_ADDR).unwrap();
        println!("Server started at {:?}", &SERVER_ADDR); 

        for connection in listener.incoming(){
            
            println!("--> Got a new connection!!");
            
            let mut buffer = [0; 512];
            let mut stream = connection.unwrap();
            
            println!("--> Start reading data");
            
            stream.read(&mut buffer).ok();
            let parsed_request = Request::parse_request(&buffer);
            
            println!("Parsed data: {:?}", parsed_request);
            
            stream.write(Response::build_dumb_response().to_str().as_bytes());
            
            println!("--> Response sent!!");
            
            stream.flush().unwrap();
        }
    }  
}
        

fn main(){
    println!("--> Starting server...");
    server::start();
}