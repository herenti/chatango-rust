
use std::net::TcpStream;
use std::io::prelude::*;
use rand::Rng;
use std::thread;
use std::time::Duration;

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



struct Chat{
    name: String,
    cumsock: TcpStream,
    wbyte: String
}

impl Chat{
    fn new(name: String, username: String, password: String) -> Self {
        let server = g_server(name.clone());
        let mut chat = Chat{
            name: name,
            cumsock: TcpStream::connect(server).unwrap(),
            wbyte: "".to_string(),
        };
        chat.cumsock.set_nonblocking(true).expect("set_nonblocking call failed");
        chat.chat_login(username, password);
        chat
    }

    fn chat_login(&mut self, username: String, password: String){

        let chat_id = rand::thread_rng().gen_range(10_u128.pow(15)..10_u128.pow(16)).to_string();
        let to_send = format!("bauth:{}:{}:{}:{}\x00", self.name, chat_id, username, password);
        println!("{}", to_send);
        let _ = self.cumsock.write(to_send.as_bytes()).unwrap();
        let mut socket_clone = self.cumsock.try_clone().expect("Failed to clone socket");
        thread::spawn(move || {
            loop {
                let data = b"\r\n\x00";
                socket_clone.write(data);
                thread::sleep(Duration::from_secs(20));

            }
        });

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
            let chat = Chat::new(i.to_string(), username.to_string(), password.to_string());

            bakery.connections.insert(bakery.connections.len(), chat);
        };
        bakery.breadbun();
        bakery

    }

    fn breadbun(&mut self){
        let mut read_sockets = vec![];
        for i in &mut self.connections {
            read_sockets.insert(read_sockets.len(), &mut i.cumsock)
        };
        let anpan_is_tasty = true;
        while anpan_is_tasty {
            for i in &mut read_sockets{
                let mut buf = [0; 1024];
                if let Ok(len) = i.read(&mut buf) {
                    if len > 0 {
                        let data = &buf[..len];
                        for x in data.split(|b| b == &0x00) {
                            let s = std::str::from_utf8(x).unwrap();
                            println!("{}", s);
                        }
                    }
                }

            }
        }
    }
}

fn main() {
    Bakery::oven("anpanbot", "", vec!["garden", "jewelisland"]);


}
