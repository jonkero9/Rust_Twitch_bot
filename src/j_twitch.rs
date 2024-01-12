use colored::{ColoredString, Colorize};
use rand::Rng;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

const PRIVATE_MSG_KEY: &str = "PRIVMSG";
const PING_KEY: &str = "PING";
const TWITCH_IRC_URL: &str = "irc.chat.twitch.tv:6667";

#[derive(Debug)]
pub struct TwitchOptions {
    pub pass: String,
    pub nick: String,
    pub join_command: String,
}

#[derive(Debug)]
struct TwitchIrcMessage {
    sender_name: String,
    message: String,
}

pub fn handle_twitch_chat(twitch_opt: TwitchOptions, user_map: &mut HashMap<String, i32>) {
    if let Ok(stream) = TcpStream::connect(TWITCH_IRC_URL) {
        twitch_stream_handler(&stream, &twitch_opt, user_map);
    }
}

fn twitch_stream_handler(
    stream: &TcpStream,
    t_opts: &TwitchOptions,
    user_map: &mut HashMap<String, i32>,
) {
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
            handle_messages(&line, &mut writer, user_map);
            line.clear();
            true
        }
        Err(e) => {
            println!("error {}", e);
            false
        }
    } {}
}

fn write_data(writer: &mut BufWriter<&TcpStream>, data: Vec<&[u8]>) {
    for b in data.iter() {
        let _ = writer.write_all(b);
        writer.flush().expect("flush err");
    }
}

fn handle_messages(
    line: &str,
    writer: &mut BufWriter<&TcpStream>,
    user_map: &mut HashMap<String, i32>,
) {
    if let Some(t_message) = check_message(line) {
        if !user_map.contains_key(&t_message.sender_name) {
            user_map.insert(
                t_message.sender_name.to_string(),
                rand::thread_rng().gen_range(0..8),
            );
        }
        println!(
            "{}: {}",
            colorize_string(
                &t_message.sender_name,
                *user_map.get(&t_message.sender_name).unwrap_or(&0)
            ),
            t_message.message
        );
    }
    if let Some(pat) = check_ping(line) {
        write_data(writer, Vec::from([format!("PONG {}\r\n", pat).as_bytes()]));
    }
}

fn check_message(message: &str) -> Option<TwitchIrcMessage> {
    if let Some(t_message) = message.split_once(PRIVATE_MSG_KEY) {
        let delin_index = t_message.0.find('!').unwrap_or(t_message.0.len());
        let sender_chunk = &t_message.0[1..delin_index].trim();

        let delin_index = t_message.1.find(':').unwrap_or(0) + 1;
        let message_chunk = &t_message.1[delin_index..].trim();

        return Some(TwitchIrcMessage {
            sender_name: sender_chunk.to_string(),
            message: message_chunk.to_string(),
        });
    }
    None
}

fn check_ping(message: &str) -> Option<&str> {
    return match message
        .find(PING_KEY)
        .and_then(|_x| message.split_once(' '))
    {
        Some(expr) => Some(expr.1),
        None => None,
    };
}

fn colorize_string(s: &str, color_code: i32) -> ColoredString {
    match color_code {
        0 => s.blue(),
        1 => s.green(),
        2 => s.yellow(),
        3 => s.red(),
        4 => s.purple(),
        5 => s.cyan(),
        6 => s.magenta(),
        7 => s.white(),
        _ => s.blue(),
    }
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

    //     #[test]
    //     fn print_test() {
    //         for pat in 0..8 {
    //             println!("{}: {}", pat, colorize_string("Hello Chat", pat));
    //         }
    //     }

    #[test]
    fn check_ping_message_returned() {
        let test_string = "PING :this.should.be.returned";
        let expected = ":this.should.be.returned";
        let result = check_ping(test_string).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn check_ping_none_return() {
        assert!(check_ping("").is_none());
        assert!(check_ping("dfadfklajdfjadfkljj").is_none());
        assert!(check_ping("").is_none());
    }
}
//
//
