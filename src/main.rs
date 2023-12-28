use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    const T_KEY: &'static str = env!("T_SEC");

    let pass = format!("PASS oauth:{}\r\n", T_KEY);
    let nick = String::from("NICK jonkero\r\n");
    let join_command = String::from("JOIN #jonkero\r\n");

    if let Ok(stream) = TcpStream::connect("irc.chat.twitch.tv:6667") {
        twitch_stream_handler(&stream, &pass, &nick, &join_command);
    }

    Ok(())
}

fn twitch_stream_handler(stream: &TcpStream, pass: &str, nick: &str, join_command: &str) {
    let mut writer = BufWriter::new(stream);
    writer.write(pass.as_bytes()).expect("error writing pass");
    writer.flush().expect("flush err");
    writer.write(nick.as_bytes()).expect("error writing nick");
    writer.flush().expect("flush err");
    writer
        .write(join_command.as_bytes())
        .expect("error writing join");
    writer.flush().expect("flush err");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    while match reader.read_line(&mut line) {
        Ok(0) => {
            println!("ok 0 {}", line);
            false
        }
        Ok(_size) => {
            println!("{}", line);
            if let Some(pat) = ping_handler(&line) {
                writer
                    .write(format!("PONG {}\r\n", pat).as_bytes())
                    .expect("err writing pong");
                writer.flush().expect("err ping flush");
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

fn ping_handler(message: &String) -> Option<&str> {
    if let Some(_index) = message.find("PING") {
        return match message.split_once(" ") {
            Some(expr) => Some(expr.1),
            None => None,
        };
    }
    None
}
