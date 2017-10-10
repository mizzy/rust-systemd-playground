extern crate dbus_bytestream;
extern crate dbus_serialize;
extern crate rustc_serialize;

use dbus_bytestream::connection::Connection;
use dbus_bytestream::message;
use dbus_serialize::decoder::DBusDecoder;
use rustc_serialize::Decoder;

fn main() {
    let conn = Connection::connect_system().unwrap();
    let mut msg = message::create_method_call("org.freedesktop.systemd1",
                                              "/org/freedesktop/systemd1",
                                              "org.freedesktop.systemd1.Manager",
                                              "GetUnit");
    msg = msg.add_arg(&"ssh.service");
    let reply = conn.call_sync(msg).unwrap().unwrap();
    let obj = reply[0].clone();
    println!("{}", DBusDecoder::new(obj).read_str().unwrap());
}
