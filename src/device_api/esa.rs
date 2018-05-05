use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::io::prelude::*;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

use discipline::*;
use session::Shot;
use device_api::api::{API, Action, DeviceCommand};

// use std::io::BufReader;

/// We use some c functions to comunicate with the ESA interface, using rust crates just did not
/// work. I just could not read data from the device.
extern {
    fn serialOpen(device: *const u8) -> i32;
    fn serialClose(fd: i32);
    fn serialWrite(fd: i32, data: *const u8, length: usize);
    fn serialRead(fd: i32, arr: *const u8, length: usize) -> usize;
}

/// Typealias for the c file desciptor
type SerialPort = i32;



#[derive(Debug)]
enum SerialError {
    OpenError,
}

#[derive(Debug)]
enum DataError {
    InvalidChecksum,
    InvalidStartOfFrame,
    InvalidEndOfFrame,
    InvalidPayload,
}



enum NopResult {
    Shot(Shot),
    Ack,
    Err(DataError),
}

// Time interval (ms) in which we search for new shots
const ESA_FETCH_INTERVAL: u64 = 1000;


/// DeviceAPI for Haering ESA.
pub struct ESA {
    /// Path to the serial device connected to the ESA interface.
    path: String,
    discipline: Discipline,
}

impl ESA {
    /// Init new DeviceAPI for ESA.
    /// path:   Path to the serial device connected to the ESA interface.
    pub fn new(path: String, discipline: Discipline) -> ESA {
        ESA { path, discipline }
    }

    /// Configure serial port to requied parameters
    /// path:   Path to the serial port device
    fn serial_open(path: String) -> Result<i32, SerialError> {
        // let port = b"/dev/ttyS0\0";
        // let path = path.push("\0");
        let port = unsafe { serialOpen(path.as_ptr()) };
        if port == -1 {
            return Result::Err(SerialError::OpenError);
        }
        return Result::Ok(port);
    }

    /// Close given serial port
    /// port:   Serial Port desciptor
    fn serial_close(port: SerialPort) {
        unsafe { serialClose(port) };
    }

    /// Write given data to port.
    /// port:       port to write to.
    /// data:       data to write.
    /// return:     given port to use it again.
    fn write(port: SerialPort, data: Vec<u8>) {
        unsafe {
            serialWrite(port, data.as_ptr(), data.len());
        }
    }

    /// Read from port.
    /// port:       port to read from.
    /// return:     tupel with
    ///                 read data
    ///                 given port to use it again.
    // TODO use Result as return/ err invalid checksum/ data
    fn read(port: SerialPort) -> Result<Vec<u8>, DataError> {
        const MAX_LEN: usize = 50;
        let raw: [u8; MAX_LEN] = [0; MAX_LEN];
        let mut read_len: usize;
        unsafe {
            read_len = serialRead(port, raw.as_ptr(), MAX_LEN);
        };

        println!("{} {:?}", read_len, raw.to_vec());
        let mut payload: Vec<u8> = Vec::new();
        for i in 0..read_len {

            match i {
                // first byte must be 0x55
                0 => match raw[i] {
                     0x55   => continue,
                     _      => return Result::Err(DataError::InvalidStartOfFrame),
                }

                // address byte, unused
                1 => continue,

                // second last byte, checksum (xor over everything befor)
                _ if i == read_len-2 => {
                    let mut buf: Vec<u8> = Vec::new();
                    buf.push(raw[0]);
                    buf.push(raw[1]);
                    buf.extend(payload.clone());
                    if ESA::calculate_checksum(buf) != raw[i] {
                        return Result::Err(DataError::InvalidChecksum);
                    }
                }

                // last byte must be 0xAA
                _ if i == read_len-1 => match raw[i] {
                    0xAA    => continue,
                    _       => return Result::Err(DataError::InvalidEndOfFrame),
                }

                // everything eles is a data byte
                _ => {
                    payload.push(raw[i]);
                }
            }
        }
        return Result::Ok(payload);
    }



    /// Calculate xor checksum over given data array
    /// data:   Data to xor
    /// return: checksum byte
    fn calculate_checksum(data: Vec<u8>) -> u8 {
        let mut checksum: u8 = 0;
        for x in &data {
            checksum ^= x;
        }
        return checksum;
    }



    /// Add start, stop and checksum bits to given payload.
    /// payload:    payload we want to send.
    /// return:     array with given payload extended with start, stop and checksum bits.
    fn form_command_data(payload: Vec<u8>) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(85);
        buf.push(1);
        buf.extend(payload);
        let checksum = ESA::calculate_checksum(buf.clone());
        buf.push(checksum);
        buf.push(170);
        return buf;
    }



    /// Send paper move command to ESA device.
    /// port:       port to sent it to.
    /// time:       time to move 0-255 (in tenths of a second).
    fn perform_band(port: SerialPort, time: u8) {
      let data = ESA::form_command_data(vec![23, time]);
      println!("perform_band");
      ESA::write(port, data);

      match ESA::read(port) {
          Ok(payload) => {
              match payload.len() {
                  1 if payload[0] == 0x08 => {
                      println!("perform_band ok");
                  }
                  _ => {
                      println!("Read Error: invalid payload: {:?}", payload);
                  }
              }
          }
          Err(err) => {
              println!("Read Error: {:?}", err);
          }
      }
    }

    /// Send NOP command to ESA device
    /// port:       port to sent it to.
    /// return:     NopResult (Shot, Nop, Error)
    fn perform_nop(port: SerialPort, target: &Target) -> NopResult {
        println!("perform_nop");

        let data = ESA::form_command_data(vec![0]);
        ESA::write(port, data);

        match ESA::read(port) {
            Ok(payload) => {
                match payload.len() {
                    1 if payload[0] == 0x08 => {
                        // Nop
                        return NopResult::Ack;
                    }
                    13 if payload[0] == 0x1D => {
                        // Trefferdaten (AuTa sendet registrierte Trefferkoordinaten)
                        let mut cursor = Cursor::new(payload);
                        let _ = cursor.read_u8().unwrap();
                        let _time = cursor.read_u32::<BigEndian>().unwrap();
                        let x = cursor.read_i32::<BigEndian>().unwrap();
                        let y = cursor.read_i32::<BigEndian>().unwrap();

                        let shot = Shot::from_cartesian_coordinates(x, y, target);
                        return NopResult::Shot(shot);
                    }
                    _ => {
                        println!("Read Error: invalid payload: {:?}", payload);
                        return NopResult::Err(DataError::InvalidPayload);
                    }
                }
            },
            Err(err) => {
                println!("Read Error: {:?}", err);
                return NopResult::Err(err);
            }
        }
    }

    /// Send config to ESA device.
    /// port:       port to sent it to.
    /// time:       time to move after each shot 0-255 (in tenths of a second).
    fn perform_set(port: SerialPort, time: u8) {
        println!("perform_set");

        let data = ESA::form_command_data(vec![20, 5, 250, 20, time, 9, 13, 8, 79, 0, 0, 0, 0, 30, 220, 1, 144]);
        ESA::write(port, data);

        match ESA::read(port) {
            Ok(payload) => {
                match payload.len() {
                    1 if payload[0] == 0x08 => {
                        println!("perform_set ok");
                    }
                    _ => {
                        println!("Read Error: invalid payload: {:?}", payload);
                    }
                }
            }
            Err(err) => {
                println!("Read Error: {:?}", err);
            }
        }
    }

}



impl API for ESA {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>) {

        // let rx = self.channel_rx;
        let serial_path = self.path.clone();
        let target = self.discipline.target.clone();
        thread::spawn(move || {

            // Sleep twice the interval time, to make shure the previous process has
            // closed the port.
            thread::sleep(Duration::from_millis(ESA_FETCH_INTERVAL*2));

            match ESA::serial_open(serial_path) {
                Ok(port) => {
                    ESA::perform_set(port, 1_u8); // Todo from discipline
                    // thread::sleep(Duration::from_millis(100));
                    ESA::perform_band(port, 1_u8); // Todo from discipline
                    // thread::sleep(Duration::from_millis(100));

                    loop {
                        match rx.try_recv() {
                            // Stop if we got a stop message or the channel disconnected
                            Ok(DeviceCommand::Stop) | Err(TryRecvError::Disconnected) => {
                                println!("Stopping DeviceAPI");
                                ESA::serial_close(port);
                                break;
                            },
                            // When we got no message we generate a shot
                            Err(TryRecvError::Empty) => {
                                match ESA::perform_nop(port, &target) {
                                    NopResult::Shot(shot) => {
                                        println!("New Shot {:?}", shot);
                                        tx.send(Action::NewShot(shot));
                                    }
                                    NopResult::Ack => {

                                    }
                                    NopResult::Err(err) => {

                                    }
                                }

                                thread::sleep(Duration::from_millis(ESA_FETCH_INTERVAL));
                            }
                            _ => {},
                        }
                    }
                },
                Err(err) => {

                    println!("init error {:?}", err);
                    // tx.send(Action:Error("err".to_string()));
                }
            }
        });

    }

}










#[cfg(test)]
mod test {
    use device_api::esa::*;

    #[test]
    fn test_calculate_checksum() {
        let checksum1 = ESA::calculate_checksum(vec![0x01, 0x02]);
        assert_eq!(0x03, checksum1);

        let checksum2 = ESA::calculate_checksum(vec![]);
        assert_eq!(0x00, checksum2);

        let checksum3 = ESA::calculate_checksum(vec![0x00, 0x00, 0xF0]);
        assert_eq!(0xF0, checksum3);

        let checksum4 = ESA::calculate_checksum(vec![0x01, 0x01, 0x02]);
        assert_eq!(0x02, checksum4);
    }



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
