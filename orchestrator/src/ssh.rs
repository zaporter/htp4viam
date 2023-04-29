use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;
pub fn ssh_test() {
    println!("Started");
    let ssh_path = PathBuf::from("/home/zack/.ssh/id_rsa_test");
    //keygen::gen_ssh_keypair(&ssh_path).unwrap();

    let ssh2_path = PathBuf::from("/home/zack/.ssh/id_rsa_orchestrator");

    // Generate a new RSA key pair with 4096 bits
    // Connect to the local SSH server
    let tcp = TcpStream::connect("rama.local:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file("zack", None, &ssh2_path, None)
        .unwrap();
    //sess.userauth_password("zack", "fr4fdsas").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
