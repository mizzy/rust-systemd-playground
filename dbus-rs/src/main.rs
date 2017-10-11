extern crate dbus;

use dbus::{Connection, BusType, Message, Path};
use dbus::arg;

fn main() {
    // Get object path by service name
    let c = Connection::get_private(BusType::System).unwrap();
    let m = Message::new_method_call("org.freedesktop.systemd1",
                                     "/org/freedesktop/systemd1",
                                     "org.freedesktop.systemd1.Manager",
                                     "GetUnit")
        .unwrap()
        .append1("ssh.service");
    let r = c.send_with_reply_and_block(m, 2000).unwrap();
    let o: Path = r.read1().unwrap();
    println!("{}", o);


    // Get active state of the service
    let m = Message::new_method_call("org.freedesktop.systemd1",
                                     o,
                                     "org.freedesktop.DBus.Properties",
                                     "Get")
        .unwrap()
        .append2("org.freedesktop.systemd1.Unit", "ActiveState");
    let r = c.send_with_reply_and_block(m, 2000).unwrap();
    let s: arg::Variant<&str> = r.read1().unwrap();
    println!("{}", s.0);
}
