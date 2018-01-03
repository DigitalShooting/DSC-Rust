use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let arc = Arc::new(Mutex::new( 0 ));
    let test1 = Test1::new(&arc);

    let test2 = Test1::new(&arc);

    test1.add();
    test2.add();
    test2.add();
    test1.add();
    test1.add();

    thread::sleep(Duration::from_secs(1));
    println!("{}", arc.lock().unwrap());
}


pub struct Test1<'a> {
    session: &'a Arc<Mutex<i32>>,
}

impl<'a> Test1<'a> {
    pub fn new(session: &Arc<Mutex<i32>>) -> Test1 {
        return Test1 { session }
    }

    pub fn add(&self) {
        let session_copy = self.session.clone();
        thread::spawn(move || {
            let mut data = session_copy.lock().unwrap();
            *data += 1;
        });

    }
}







    // // Create channels for sending and receieving
    // let (one_tx, one_rx) = channel();
    // let (three_tx, three_rx) = channel();
    //
    // // Spawn one second timer
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(Duration::from_secs(1));
    //         one_tx.send("tick").unwrap();
    //     }
    // });
    //
    // // Spawn three second timer
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(Duration::from_secs(3));
    //         three_tx.send("tock").unwrap();
    //     }
    // });
    //
    // loop {
    //     thread::sleep(Duration::from_millis(50));
    //     let _ = one_rx.try_recv().map(|reply| println!("{}", reply));
    //     let _ = three_rx.try_recv().map(|reply| println!("{}", reply));
    //     return;
    // }










    // fn typed_example() -> Result<(), Error> {
    //     // Some JSON input data as a &str. Maybe this comes from the user.
    //     let data = r#"{
    //                     "parts": [
    //                         {
    //                             "series": [
    //                                 {
    //                                     "shots": [
    //                                         {
    //                                             "teiler": 14.2,
    //                                             "angle": 40.2
    //                                         }
    //                                     ]
    //                                 }
    //                             ],
    //                             "part_type": "probe"
    //                         }
    //                     ],
    //                     "user": {
    //                         "first_name": "Jannik",
    //                        "last_name": "Lorenz",
    //                        "id": "0"
    //                     },
    //                     "club": {
    //                        "name": "demoClub",
    //                        "id": "1"
    //                     },
    //                     "team": {
    //                         "name": "demoTeam",
    //                         "id": "2"
    //                     }
    //                   }"#;
    //
    //     // Parse the string of data into a Person object. This is exactly the
    //     // same function as the one that produced serde_json::Value above, but
    //     // now we are asking it for a Person as output.
    //     let session: Session = serde_json::from_str(data)?;
    //
    //     // Do things just like with any other Rust data structure.
    //     println!("User: {} {}, id: {}/ Club: {}/ Team: {}", session.user.first_name, session.user.last_name, session.user.id, session.club.name, session.team.name);
    //
    //     Ok(())
    // }
