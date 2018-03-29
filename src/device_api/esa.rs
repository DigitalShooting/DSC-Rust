use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::env;
use std::io;
use std::error::Error;


use std::io::prelude::*;
use serial::prelude::*;
use serial;
use serial::SystemPort;



use session::*;
use dsc_manager::*;

use helper;

use api::API;
use api::Action;



/*
dsc_manager -> shot_provider
- stop signal
==> Erzeugt beim aufruf von start, returns tx

shot_provider -> dsc_manager
- onNewShot
- onError
==> Erzeugt von dsc_manager

*/




pub struct ESA {
    // port: Option<SystemPort>,
    path: String,
}

impl ESA {
    pub fn new(path: String) -> ESA {
        ESA { path }
    }

    fn initSerial(path: String) -> serial::Result<SystemPort> {
        let mut port = serial::open(&path)?;

        try!(port.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud9600));
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        }));

        try!(port.set_timeout(Duration::from_millis(1000)));

        Ok(port)
    }

    /**
     Add start/ stop bits and checksum to given payload
     @param payload     payload we want to send
     */
    fn form_command_data(payload: Vec<u8>) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(85);
        buf.push(1);
        buf.extend(payload);
        let mut checksum: u8 = 0;
        for x in &buf {
            checksum ^= x;
        }
        buf.push(checksum);
        buf.push(170);
        return buf;
    }

    fn write(mut port: SystemPort, data: Vec<u8>) -> SystemPort {
        port.set_rts(false); // Disable RTS to send
        port.write(&data); // Write data
        return port;
    }

    fn read(mut port: SystemPort) -> Option<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        port.set_rts(true); // Enable RTS to send
        port.read(&mut buf);
        Some(buf)
    }


    // fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    //     let mut buf: Vec<u8> = (0..255).collect();
    //
    //     try!(port.write(&buf[..]));
    //     try!(port.read(&mut buf[..]));
    //
    //     Ok(())
    // }

    // Send paper move to serial device
    // unsigned char time:    Time to move (0-255)
    fn perform_band(mut port: SystemPort, time: u8) -> SystemPort {
      let data = ESA::form_command_data(vec![23, time]);
      port = ESA::write(port, data);
      // readFromHaering(fd, 17);
      return port;
    }



    // Send NOP to serial device
    // output:     Recived bytes
    fn perform_nop(mut port: SystemPort) -> SystemPort {
        let data = ESA::form_command_data(vec![19, 0]);
        port = ESA::write(port, data);
        // readFromHaering(fd, 17);
        return port;
    }



    // Send Config to serial device
    // unsigned char time:   Time to move after each shot (0-255)
    fn perform_set(mut port: SystemPort, time: u8) -> SystemPort {
      let data = ESA::form_command_data(vec![20, 5, 250, 20, time, 9, 13, 8, 79, 0, 0, 0, 0, 30, 220, 1, 144]);
      port = ESA::write(port, data);
      // writeToHaering(fd, seq, sizeof(seq));
      // readFromHaering(fd, 0);
      return port;
    }

}



impl API for ESA {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<Action>) {

        // let rx = self.channel_rx;
        let serial_path = self.path.clone();
        thread::spawn(move || {

            match ESA::initSerial(serial_path) {
                Ok(mut port) => {
                    port = ESA::perform_band(port, 1_u8);
                    loop {
                        match rx.try_recv() {
                            // Stop if we got a stop message or the channel disconnected
                            Ok(Action::Stop) | Err(TryRecvError::Disconnected) => {
                                println!("Stopping DeviceAPI");
                                break;
                            },
                            // When we got no message we generate a shot
                            Err(TryRecvError::Empty) => {
                                port = ESA::perform_nop(port);
                                thread::sleep(Duration::from_millis(250));
                            }
                            _ => {},
                        }
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    // tx.send(Action:Error("err".to_string()));
                }
            }
        });


    }




    fn stop(&self) {

    }

}










#[cfg(test)]
mod test {
    use esa::*;

    #[test]
    fn test_form_command_data_band() {
        let buf = ESA::form_command_data(vec![23, 2]);
        let buf_expected: Vec<u8> = vec![85, 1, 23, 2, 65, 170];
        assert_eq!(buf_expected, buf);
    }

    #[test]
    fn test_form_command_data_nop() {
        let buf = ESA::form_command_data(vec![19, 0]);
        let buf_expected: Vec<u8> = vec![85, 1, 19, 0, 71, 170];
        assert_eq!(buf_expected, buf);
    }
}
