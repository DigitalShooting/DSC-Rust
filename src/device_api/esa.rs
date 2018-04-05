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

use session::Shot;
use api::{API, Action, DeviceCommand};
use helper;



/// DeviceAPI for Haering ESA.
pub struct ESA {
    /// Path to the serial device connected to the ESA interface.
    path: String,
}

impl ESA {
    /// Init new DeviceAPI for ESA.
    /// path:   Path to the serial device connected to the ESA interface.
    pub fn new(path: String) -> ESA {
        ESA { path }
    }

    /// Configure serial port to requied parameters
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

    /// Add start, stop and checksum bits to given payload.
    /// payload:    payload we want to send.
    /// return:     array with given payload extended with start, stop and checksum bits.
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

    /// Write given data to port.
    /// port:       port to write to.
    /// data:       data to write.
    /// return:     given port to use it again.
    fn write(mut port: SystemPort, data: Vec<u8>) -> SystemPort {
        // Disable RTS to send
        port.set_rts(false);
        // Write data
        port.write(&data);
        return port;
    }

    /// Read from port.
    /// port:       port to read from.
    /// return:     tupel with
    ///                 read data
    ///                 given port to use it again.
    fn read(mut port: SystemPort) -> (Vec<u8>, SystemPort) {
        let mut buf: Vec<u8> = Vec::new();
        // Enable RTS to send
        port.set_rts(true);
        let size = port.read_to_end(&mut buf);
        println!("{:?} {:?}", size, buf);

        return (buf, port);
    }


    // fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    //     let mut buf: Vec<u8> = (0..255).collect();
    //
    //     try!(port.write(&buf[..]));
    //     try!(port.read(&mut buf[..]));
    //
    //     Ok(())
    // }

    /// Send paper move command to ESA device.
    /// port:       port to sent it to.
    /// time:       time to move 0-255 (in tenths of a second).
    /// return:     given port to use it again.
    fn perform_band(mut port: SystemPort, time: u8) -> SystemPort {
      let data = ESA::form_command_data(vec![23, time]);
      port = ESA::write(port, data);
      let (readResult, port) = ESA::read(port); //readFromHaering(fd, 17);
      // TODO ckeck
      return port;
    }



    /// Send NOP command to ESA device
    /// port:       port to sent it to.
    /// return:     given port to use it again.
    fn perform_nop(mut port: SystemPort) -> SystemPort {
        let data = ESA::form_command_data(vec![19, 0]);
        port = ESA::write(port, data);
        let (readResult, port) = ESA::read(port); // readFromHaering(fd, 17);
        // TODO Read
        return port;
    }



    /// Send config to ESA device.
    /// port:       port to sent it to.
    /// time:       time to move after each shot 0-255 (in tenths of a second).
    /// return:     given port to use it again.
    fn perform_set(mut port: SystemPort, time: u8) -> SystemPort {
      let data = ESA::form_command_data(vec![20, 5, 250, 20, time, 9, 13, 8, 79, 0, 0, 0, 0, 30, 220, 1, 144]);
      port = ESA::write(port, data); // writeToHaering(fd, seq, sizeof(seq));
      let (readResult, port) = ESA::read(port); // readFromHaering(fd, 0);
      // TODO Read
      return port;
    }

}



impl API for ESA {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>) {

        // let rx = self.channel_rx;
        let serial_path = self.path.clone();
        thread::spawn(move || {

            match ESA::initSerial(serial_path) {
                Ok(mut port) => {
                    port = ESA::perform_band(port, 1_u8);
                    loop {
                        match rx.try_recv() {
                            // Stop if we got a stop message or the channel disconnected
                            Ok(DeviceCommand::Stop) | Err(TryRecvError::Disconnected) => {
                                println!("Stopping DeviceAPI");
                                break;
                            },
                            // When we got no message we generate a shot
                            Err(TryRecvError::Empty) => {
                                port = ESA::perform_nop(port);
                                thread::sleep(Duration::from_millis(1000));
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
