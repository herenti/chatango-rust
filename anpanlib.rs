/*
 * TODO: CLEAN UP CODE, ADD MORE EVENTS, REDUCE RELIANCE ON CLONING/RESTRUCTURE CODE (possibly done). COMMANDS AS SEPERATE MODULE.
 * This is a fully functional chatango library written in rust.
 * I am a newbie coder to rust, so the code may be sloppy. If someone wants to add suggestions for the code structure, contact me on discord @herenti.
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
use std::sync::{Arc, Mutex};
use serde_json;



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

fn youtube(search: &str) -> String {
    let url = format!("https://www.googleapis.com/youtube/v3/search?q={}&key=AIzaSyBPXEemy53RKTrOAsCN-UPCRhfARbyvKs0&type=video&maxResults=1&part=snippet", search);
    let res = reqwest::blocking::get(url).expect("REASON").text().unwrap();
    let data: serde_json::Value = serde_json::from_str(&res).unwrap();
    let result = if data["items"][0]["id"]["videoId"].as_str().is_some(){
        let _id = &data["items"][0]["id"]["videoId"].as_str().unwrap();
        let _title = &data["items"][0]["snippet"]["title"].as_str().unwrap();
        format!("https://www.youtube.com/watch?v={}\r\r\r\rVideo title [<b>{}</b>]", _id, _title)
    } else {
        "No video found.".to_string()
    };
    result


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
    byteready: bool,
    username: String,
    password: String,
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
            username,
            password,
        };

        chat.cumsock.set_nonblocking(true).expect("set_nonblocking call failed");
        if ctype == "chat" {
            chat.chat_login();
        } else {
            chat.pm_login();
        };
        chat
    }

    fn chat_login(&mut self){

        let chat_id = rand::rng().random_range(10_u128.pow(15)..10_u128.pow(16)).to_string();
        self.chat_send(vec!["bauth", &self.name.clone(), &chat_id, &self.username.clone(), &self.password.clone()]);
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

    fn pm_login(&mut self){

        let auth = auth(&self.username.clone(), &self.password.clone());
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




}


struct Bakery{
    connections: Vec<Chat>,
    current_chat: String,
    to_send_room: String,
    username: String,
    password: String,
    font_color: String,
    name_color: String,
    font_size: i32,

}

impl Bakery{

    fn oven(username: &str, password: &str, room_list: Vec<&str>) -> Self {
        let mut bakery = Bakery {
            connections: vec![],
            current_chat: "None".to_string(),
            to_send_room: "None".to_string(),
            username: username.to_string(),
            password: password.to_string(),
            name_color: "C7A793".to_string(),
            font_color: "F7DCCE".to_string(),
            font_size: 10,
        };
        for i in room_list{
            let chat = Chat::new(i.to_string(), username.to_string(), password.to_string(), "chat");

            bakery.connections.push(chat);
        };
        let chat = Chat::new("_pm".to_string(), username.to_string(), password.to_string(), "pm");
        bakery.connections.push(chat);




        bakery

    }

    fn chat_post(&mut self, args: &str){
        let room = if self.to_send_room != "None".to_string(){
            &self.to_send_room
        }
        else {
            &self.current_chat
        };
        let message  = format!("<n{}/><f x{}{}=\"0\">{}</f>\r\n\x00", &self.name_color, &self.font_size, &self.font_color,  args);
        for i in &mut self.connections {
            if i.name == room.to_string() {
                i.chat_send(vec!["bm","fuck", "2048", &message]);
            }
        }

    }

    fn send_to_chat(&mut self, room: &str, args: &str){
        for i in &mut self.connections{
            if &i.name == room {
                self.to_send_room = room.to_string();
            };
        };
        if self.to_send_room != "None".to_string(){
            self.chat_post(&args);
            self.to_send_room = "None".to_string();
            self.chat_post("Done.");
        }
        else {
            self.chat_post("I am not in that chat.")
        }


    }

    fn chat_join(&mut self, args: &str) {
        let chat = Chat::new(args.to_string(), self.username.clone(), self.password.clone(), "chat");
        self.connections.push(chat);
    }

    fn chat_send(&mut self, data: Vec<&str>){
        for i in &mut self.connections {
            if i.name == self.current_chat {
                i.chat_send(data.clone());
            }
        }

    }

    fn events(&mut self, chatname: &str, collection: Vec<&str>){
        let event = &collection[0];
        let data = &collection[1..];
        self.current_chat = chatname.to_string();
        //println!("event: {:?} data: {:?}", event, data);
        if *event == "b"{
            self.event_b(data);
        }
        if *event == "inited"{
            self.event_inited(data);
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
            chat: self.current_chat.clone(),
        };

        self.on_post(message);


    }

    fn event_inited(&mut self, data: &[&str]){
        self.chat_send(vec!["getpremium", "1"]);
        self.chat_send(vec!["g_participants", "start"]);
        self.chat_send(vec!["getbannedwords"]);
        self.chat_send(vec!["msgbg", "1"]);
        println!("logged into: {}", &self.current_chat);
    }


    fn on_post(&mut self, message: Message){
        //println!("{}: {}", message.user, message.content);
        if message.content.to_lowercase().contains("herenti"){
            println!("{}: {}: {}", message.user, message.chat, message.content)
        }
        if message.chat != "".to_string(){
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
                self.commands(message, &command, &args);

            }
        }
    }

    fn commands(&mut self, message: Message, command: &str, args: &str){
        let mods = vec![""];
        let user = message.user.as_str();
        let ismod = if mods.contains(&user) {
            true
        } else {
            false
        };
        match command {
            "say" => {
                self.chat_post(&args);
            }
            "yt" => {
                self.chat_post(&youtube(&args));
            }
            "rainbow" => {
                let size = "12";
                let rainbowed = Rainbow::rainbow_text(&args, size);
                if rainbowed.len() > 2490{
                    self.chat_post("That message is too long for chatango.");
                } else{
                    self.chat_post(&rainbowed);
                }
            }

            "send" => {
                if ismod {
                    let args = args.split(" ");
                    let args = args.collect::<Vec<&str>>();
                    let room = args[0];
                    let message = args[1..].join(" ");
                    self.send_to_chat(room, &message);
                } else {
                    self.chat_post("You do not have permission to use this command.");
                }
            }
            "join" => {
                if ismod {
                    self.chat_join(&args);
                    self.chat_post("Done.");
                } else {
                    self.chat_post("You do not have permission to use this command.");
                }
            }
            "rsend" => {
                if ismod {
                    let args = args.split(" ");
                    let args = args.collect::<Vec<&str>>();
                    let room = args[0];
                    let message = args[1..].join(" ");
                    let size = "12";
                    let rainbowed = Rainbow::rainbow_text(&message, size);
                    if rainbowed.len() > 2490{
                        self.chat_post("That message is too long for chatango.");
                    } else{
                        self.send_to_chat(room, &rainbowed);
                    }
                } else {
                    self.chat_post("You do not have permission to use this command.");
                }
            }

            _ => {

                self.chat_post("Unknown command");
            }

        }
    }


}

fn main() {

    let mut bakery = Bakery::oven("", "", vec![""]);

    breadbun(&mut bakery);

    fn breadbun(bakery: &mut Bakery) {
        let anpan_is_tasty = true;
        while anpan_is_tasty {
            let mut cloned_conn = vec![];
            for i in &mut bakery.connections{
                cloned_conn.push((i.name.clone(), i.cumsock.try_clone().expect("failed to clone socket.")));
            };
            for (name, mut con) in cloned_conn {
                let mut buf = [0; 1024];
                if let Ok(len) = con.read(&mut buf) {
                    if len > 0 {
                        let data = &buf[..len];
                        for x in data.split(|b| b == &0x00) {
                            let s = String::from_utf8_lossy(x);
                            let s = s.trim();
                            let s = s.split(":");
                            let collection = s.collect::<Vec<&str>>();
                            bakery.events(&name.clone(), collection);
                        }
                    }
                }

            }
        }
    }


}
