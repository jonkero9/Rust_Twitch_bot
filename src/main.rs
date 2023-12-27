use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    const T_KEY: &'static str = env!("T_SEC");

    let pass = format!("PASS oauth:{}\r\n", T_KEY);
    let nick = String::from("NICK jonkero\r\n");
    let join_command = String::from("JOIN #jonkero\r\n");

    println!("{}{}{}", pass, nick, join_command);
    if let Ok(stream) = TcpStream::connect("irc.chat.twitch.tv:6667") {
        let mut writer = BufWriter::new(&stream);

        writer.write(pass.as_bytes()).expect("error writing pass");
        writer.flush().expect("flush err");
        writer.write(nick.as_bytes()).expect("error writing nick");
        writer.flush().expect("flush err");
        writer
            .write(join_command.as_bytes())
            .expect("error writing join");
        writer.flush().expect("flush err");

        println!("writing done");
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        while match reader.read_line(&mut line) {
            Ok(0) => {
                println!("ok 0 {}", line);
                false
            }
            Ok(_size) => {
                println!("{}", line);
                line.clear();
                true
            }
            Err(e) => {
                println!("error {}", e);
                false
            }
        } {}
    }
    Ok(())
}
