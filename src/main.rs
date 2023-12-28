use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

#[derive(Debug)]
struct TwitchOptions<'a> {
    pass: &'a str,
    nick: &'a str,
    join_command: &'a str,
}

#[derive(Debug)]
struct TwitchIrcMessage<'a> {
    sender_name: &'a str,
    message: &'a str,
}

fn main() -> std::io::Result<()> {
    //const T_KEY: &'static str = env!("T_SEC");
    const T_KEY: &'static str = "dfsdf";

    let twitch_opt = TwitchOptions {
        pass: &format!("PASS oauth:{}\r\n", T_KEY),
        nick: "NICK jonkero\r\n",
        join_command: "JOIN #jonkero\r\n",
    };

    //    if let Ok(stream) = TcpStream::connect("irc.chat.twitch.tv:6667") {
    //        twitch_stream_handler(&stream, &twitch_opt);
    //    }

    if let Ok(stream) = TcpStream::connect("127.0.0.1:3333") {
        twitch_stream_handler(&stream, &twitch_opt);
    }
    Ok(())
}

fn twitch_stream_handler(stream: &TcpStream, t_opts: &TwitchOptions) {
    let mut writer = BufWriter::new(stream);
    write_data(
        &mut writer,
        Vec::from([
            t_opts.pass.as_bytes(),
            t_opts.nick.as_bytes(),
            t_opts.join_command.as_bytes(),
        ]),
    );

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    while match reader.read_line(&mut line) {
        Ok(0) => {
            println!("ok 0 {}", line);
            false
        }
        Ok(_size) => {
            if let Some(t_message) = check_message(&line) {
                println!("{}: {}", t_message.sender_name, t_message.message);
            }
            if let Some(pat) = check_ping(&line) {
                write_data(
                    &mut writer,
                    Vec::from([format!("PONG {}\r\n", pat).as_bytes()]),
                );
            }
            line.clear();
            true
        }
        Err(e) => {
            println!("error {}", e);
            false
        }
    } {}
}

fn check_message(message: &String) -> Option<TwitchIrcMessage> {
    const THE_KEY: &'static str = "PRIVMSG";
    if let Some(_index) = message.find(THE_KEY) {
        let parsed_data: Vec<&str> = message.split(THE_KEY).collect();
        if parsed_data.len() >= 2 {
            return Some(TwitchIrcMessage {
                sender_name: parsed_data.first().expect("checked len"),
                message: parsed_data.last().expect("checked len"),
            });
        }
    }
    None
}

fn write_data(writer: &mut BufWriter<&TcpStream>, data: Vec<&[u8]>) {
    for b in data.iter() {
        writer.write(b).expect("error writing pass");
        writer.flush().expect("flush err");
    }
}

fn check_ping(message: &String) -> Option<&str> {
    if let Some(_index) = message.find("PING") {
        return match message.split_once(" ") {
            Some(expr) => Some(expr.1),
            None => None,
        };
    }
    None
}
