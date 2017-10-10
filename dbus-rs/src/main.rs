extern crate dbus;

use dbus::{Connection, BusType, Message, MessageItem};
use dbus::arg;
use dbus::obj::ObjectPath;

fn main() {
    let c = Connection::get_private(BusType::System).unwrap();
    let m = Message::new_method_call("org.freedesktop.systemd1",
                                     "/org/freedesktop/systemd1",
                                     "org.freedesktop.systemd1.Manager",
                                     "GetUnit")
        .unwrap()
        .append1("ssh.service");
    let r = c.send_with_reply_and_block(m, 2000).unwrap();
    let o: arg::Path = r.read1().unwrap();

    println!("{:?}", r);
}
