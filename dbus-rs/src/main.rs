extern crate dbus;

use dbus::{Connection, BusType, Message, Path};

fn main() {
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
}
