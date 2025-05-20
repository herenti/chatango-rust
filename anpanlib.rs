/*
 TODO: CLEAN UP CODE AND ADD EVENTS.
*/


use std::net::TcpStream;
use std::io::prelude::*;
use rand::Rng;
use std::thread;
use std::time::Duration;
use regex::Regex;
use std::collections::HashMap;
use reqwest::header::USER_AGENT;
use reqwest::header::HeaderValue;
use html_escape::encode_text;
mod rainbow;
use rainbow::Rainbow; //found in extra-stuff repository. i do not own this code.



fn g_server(mut group: String) -> String{

    let weights = [[5, 75],[6, 75],[7, 75],[8, 75],[16, 75],[17, 75],[18, 75],[9, 95],[11, 95],[12, 95],[13, 95],[14, 95],[15, 95],[19, 110],[23, 110],[24, 110],[25, 110],[26, 110],[28, 104],[29, 104],[30, 104],[31, 104],[32, 104],[33, 104],[35, 101],[36, 101],[37, 101],[38, 101],[39, 101],[40, 101],[41, 101],[42, 101],[43, 101],[44, 101],[45, 101],[46, 101],[47, 101],[48, 101],[49, 101],[50, 101],[52, 110],[53, 110],[55, 110],[57, 110],[58, 110],[59, 110],[60, 110],[61, 110],[62, 110],[63, 110],[64, 110],[65, 110],[66, 110],[68, 95],[71, 116],[72, 116],[73, 116],[74, 116],[75, 116],[76, 116],[77, 116],[78, 116],[79, 116],[80, 116],[81, 116],[82, 116],[83, 116],[84, 116]];

    group = group.replace("-","q").replace("_","q");
    let a = if group.len() > 6 {
        let a = if group.len() >= 9 {
            &group[6..9]
        } else {
            &group[6..]
        };
        let a = i64::from_str_radix(a, 36).unwrap() as f64;
        let a = f64::max(1000.0, a);
        a
    }
    else{
        1000.0
    };
    let b = std::cmp::min(5, group.len());
    let b = &group[..b];
    let b = i64::from_str_radix(b, 36).unwrap() as f64;
    let num = (b / a) % 1.0;
    let mut anpan = 0.0;
    let mut s_number = 0;
    let total_weight: f64 = weights.iter().map(|a| a[1] as f64).sum();
    for x in weights {
        anpan += x[1] as f64 / total_weight;
        if num <= anpan {
            s_number += x[0];
            break;
        }
    }

    format!("s{}.chatango.com:443", s_number)
}

fn auth(user: &str, pass: &str) -> String {
    let mut form = HashMap::new();
    form.insert("user_id", user);
    form.insert("password", pass);
    form.insert("storecookie", "on");
    form.insert("checkerrors", "yes");
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://chatango.com/login")
    .form(&form)
    .header(USER_AGENT, HeaderValue::from_static(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"
    ))
    .send();

    let res = res.unwrap();
    let res = res.headers();
    let cookie: Vec<_> = res.get_all("set-cookie").iter().filter_map(|val| val.to_str().ok()).map(|s| s.to_string()).collect();
    let cookie = &cookie[2];
    let re = Regex::new(r"auth.chatango.com=(.*?);").unwrap();
    let extract = re.captures(cookie).unwrap().get(1);
    let extract = extract.unwrap().as_str().to_string();
    extract
}


struct Message{
    user: String,
    cid: String,
    uid: String,
    time: String,
    sid: String,
    ip: String,
    content: String,
    chat: String,
}

struct Chat{
    name: String,
    cumsock: TcpStream,
    wbyte: String,
    byteready: bool
}

impl Chat{
    fn new(name: String, username: String, password: String, ctype: &str) -> Self {
        let server = if ctype == "chat" {
        g_server(name.clone())
        } else {
            let server = "c1.chatango.com:5222".to_string();
            server
        };
        let mut chat = Chat{
            name: name,
            cumsock: TcpStream::connect(server).unwrap(),
            wbyte: "".to_string(),
            byteready: false,
        };
        chat.cumsock.set_nonblocking(true).expect("set_nonblocking call failed");
        if ctype == "chat" {
        chat.chat_login(username, password);
        } else {
            chat.pm_login(username, password);
        };
        chat
    }

    fn chat_login(&mut self, username: String, password: String){

        let chat_id = rand::thread_rng().gen_range(10_u128.pow(15)..10_u128.pow(16)).to_string();
        self.chat_send(vec!["bauth", &self.name.clone(), &chat_id, &username, &password]);
        self.byteready = true;
        let mut socket_clone = self.cumsock.try_clone().expect("Failed to clone socket");
        thread::spawn(move || {
            loop {
                let data = b"\r\n\x00";
                socket_clone.write(data);
                thread::sleep(Duration::from_secs(20));

            }
        });

    }

    fn pm_login(&mut self, username: String, password: String){

        let auth = auth(&username, &password);
        let to_send = format!("tlogin:{}:2\x00", auth);
        let _ = self.cumsock.write(to_send.as_bytes()).unwrap();
        let mut socket_clone = self.cumsock.try_clone().expect("Failed to clone socket");
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(20));
                let data = b"\r\n\x00";
                socket_clone.write(data);
            }
        });

    }

    fn chat_send(&mut self, data: Vec<&str>){
        let ending = if self.byteready {
            "\r\n\x00"
        } else{
            "\x00"
        };
        let data = data.join(":");
        let data = format!("{}{}", data, ending);
        self.cumsock.write(data.as_bytes());

    }

    fn chat_post(&mut self, args: &str){
        let message  = format!("<n000000/><f x12000000=\"0\">{}</f>\r\n\x00", args);
        self.chat_send(vec!["bm","fuck", "2048", &message]);

    }

    fn events(&mut self, collection: Vec<&str>){
        let event = &collection[0];
        let data = &collection[1..];
        //println!("event: {:?} data: {:?}", event, data);
        if *event == "b"{
            self.event_b(data);

        }
    }

    fn event_b(&mut self, data: &[&str]){
        let user = data[1];
        let alias = data[2];
        let user = if user == "" {
            "None"
        } else{
            user
        };
        let user = if user == "None"{
            if alias == ""{
                "None"
            } else if alias == "None"{
                "None"
            } else{
                alias
            }
        }else
        { user};
        let re = Regex::new(r"<.*?>").unwrap();
        let content = data[9..].join("");
        let content = re.replace_all(&content, "");
        let content = html_escape::decode_html_entities(&content);
        let mut message = Message{
            user: user.to_string(),
            cid: data[4].to_string(),
            uid: data[3].to_string(),
            time: data[0].to_string(),
            sid: data[5].to_string(),
            ip: data[6].to_string(),
            content: content.to_string(),
            chat: self.name.clone(),
        };

        self.on_post(message);


    }

    fn on_post(&mut self, message: Message){
        //println!("{}: {}", message.user, message.content);
        if message.content.to_lowercase().contains("herenti"){
            println!("{}: {}: {}", message.user, message.chat, message.content)
        }
        if message.chat != "jewelisland".to_string(){
            if message.content.starts_with("$") {
                let args = message.content.split(" ");
                let args: Vec<&str> = args.collect();
                let command = args[0];
                let args = if args.len() > 1 {
                    args[1..].join(" ")
                } else {
                    "".to_string()
                };
                let command = command.replace("$", "");
                let command = command.to_lowercase();
                match command.as_str() {
                    "say" => {
                        self.chat_post(&args);
                    }
                    "rainbow" => {
                        let size = "12";
                        let rainbowed = Rainbow::rainbow_text(&args, size);
                        self.chat_post(&rainbowed);
                    }
                    _ => {

                       self.chat_post("Unknown command");
                    }

                }



            }
        }
    }


}


struct Bakery{
    connections: Vec<Chat>,

}

impl Bakery{

    fn oven(username: &str, password: &str, room_list: Vec<&str>) -> Self {
       let mut bakery = Bakery {
            connections: vec![],
        };
        for i in room_list{
            let chat = Chat::new(i.to_string(), username.to_string(), password.to_string(), "chat");

            bakery.connections.insert(bakery.connections.len(), chat);
        };
        let chat = Chat::new("_pm".to_string(), username.to_string(), password.to_string(), "pm");
        bakery.connections.insert(bakery.connections.len(), chat);
        bakery.breadbun();
        bakery

    }

    fn breadbun(&mut self){
        let anpan_is_tasty = true;
        while anpan_is_tasty {
            for con in &mut self.connections{
                    let mut buf = [0; 1024];
                    if let Ok(len) = con.cumsock.read(&mut buf) {
                        if len > 0 {
                            let data = &buf[..len];
                            for x in data.split(|b| b == &0x00) {
                                let s = String::from_utf8_lossy(x);
                                let s = s.trim();
                                let s = s.split(":");
                                let collection = s.collect::<Vec<&str>>();
                                con.events(collection);
                            }
                        }
                    }

            }
        }
    }
}

fn main() {
    Bakery::oven("anpanbot", "", vec!["princess-garden","jewelisland", "epic"]);


}
