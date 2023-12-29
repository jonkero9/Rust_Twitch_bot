use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

use colored::Colorize;

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

fn main() {
    const T_KEY: &'static str = env!("T_SEC");

    let twitch_opt = TwitchOptions {
        pass: &format!("PASS oauth:{}\r\n", T_KEY),
        nick: "NICK jonkero\r\n",
        join_command: "JOIN #jonkero\r\n",
    };

    if let Ok(stream) = TcpStream::connect("irc.chat.twitch.tv:6667") {
        twitch_stream_handler(&stream, &twitch_opt);
    }
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
                println!("{}: {}", t_message.sender_name.green(), t_message.message);
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

fn check_message(message: &str) -> Option<TwitchIrcMessage> {
    const THE_KEY: &'static str = "PRIVMSG";
    if let Some(t_message) = message.split_once(THE_KEY) {
        let delin_index = t_message.0.find("!").unwrap_or(t_message.0.len());
        let sender_chunk = &t_message.0[1..delin_index].trim();

        let delin_index = t_message.1.find(":").unwrap_or(0) + 1;
        let message_chunk = &t_message.1[delin_index..].trim();

        return Some(TwitchIrcMessage {
            sender_name: sender_chunk,
            message: message_chunk,
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_message_edge_cases() {
        let test_string = "";
        assert!(check_message(test_string).is_none());
        let test_string = "dfadfadfadfadf adfadfadf adfad";
        assert!(check_message(test_string).is_none());
    }

    #[test]
    fn check_message_message_parsed() {
        let test_string = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar \
                           :bleedPurple spider, metensis future 0x29  ";
        let expected_sender = "foo";
        let expected_message = "bleedPurple spider, metensis future 0x29";
        let result = check_message(test_string).unwrap();
        assert_eq!(expected_sender, result.sender_name);
        assert_eq!(expected_message, result.message);
    }
}

//
