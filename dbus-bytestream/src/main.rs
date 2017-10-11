extern crate dbus_bytestream;
extern crate dbus_serialize;
extern crate rustc_serialize;

use dbus_bytestream::connection::Connection;
use dbus_bytestream::message;
use dbus_serialize::decoder::DBusDecoder;
use dbus_serialize::types;
use rustc_serialize::Decoder;

fn main() {
    let conn = Connection::connect_system().unwrap();
    let mut msg = message::create_method_call("org.freedesktop.systemd1",
                                              "/org/freedesktop/systemd1",
                                              "org.freedesktop.systemd1.Manager",
                                              "GetUnit");
    msg = msg.add_arg(&"ssh.service");
    let reply = conn.call_sync(msg).unwrap().unwrap();
    let object_path = DBusDecoder::new(reply[0].clone()).read_str().unwrap();

    println!("{}", object_path);

    let msg = message::create_method_call("org.freedesktop.systemd1",
                                          &object_path,
                                          "org.freedesktop.DBus.Properties",
                                          "Get")
        .add_arg(&"org.freedesktop.systemd1.Unit")
        .add_arg(&"ActiveState");

    let reply = conn.call_sync(msg).unwrap().unwrap();
    let active_state: types::Variant = DBusDecoder::decode(reply[0].clone()).unwrap();
    println!("{}", active_state);


}


// [Variant(Variant { object: BasicValue(String("active")), signature: Signature("s") })]
