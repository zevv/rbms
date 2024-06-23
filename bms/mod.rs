

pub mod evq;
pub mod rv;
pub mod dev;

use evq::Event;

pub fn bms() {

    let evq = evq::Evq::new();
    let mut mgr = dev::Mgr::new();

    let g = dev::gpio::dummy::new(evq, 1);
    mgr.add(g);

    let g2 = dev::gpio::dummy::new(evq, 2);
    mgr.add(g2);

    let uart = dev::uart::linux::new(evq, "/dev/stdout");
    mgr.add(uart);

    uart.write("=== Hello\n".as_bytes());

    mgr.init();
    mgr.dump();

    let h = g;

    g.set(false);
    h.set(true);

    evq.reg(|e| {
        match e {
            Event::Tick1Hz => {
                println!("1Hz");
            }
            Event::Tick10Hz => {
                println!("10Hz");
            }
            Event::Uart { data, len: _ } => {
                println!("Uart {:?}", data);
            }
        }
    });


    evq.run();
}

