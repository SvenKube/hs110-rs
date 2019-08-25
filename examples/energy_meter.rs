use hs110::SmartPlug;
use std::thread;
use std::time::Duration;

fn main() {
    let plug_ip = String::from("192.168.178.97:9999");
    let plug = SmartPlug::new(plug_ip);

    let p1 = plug.clone();
    let p2 = plug.clone();

    //let sysinfo = plug.get_sysinfo();
    //plug.turn_on();
    //plug.turn_off();

    let child = thread::spawn(move || {
        loop {
            let res = p1.get_emeter_realtime();
            let realtime_stats = res.unwrap()
                .emeter.unwrap()
                .get_realtime.unwrap();

            println!("Current realtime stats: {:#?}", realtime_stats);

            thread::sleep(Duration::from_secs(1));
        }
    });

    let child2 = thread::spawn(move || {
        loop {
            p2.turn_on();
            thread::sleep(Duration::from_secs(20));

            p2.turn_off();
            thread::sleep(Duration::from_secs(20));
        }
    });

    let _ = child.join();
    let _ = child2.join();
}
