



#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub id: String,
}

impl User {
    pub fn empty() -> User {
        User {
            first_name: String::from(""),
            last_name: String::from(""),
            id: String::from(""),
        }
    }

    pub fn guest() -> User {
        User {
            first_name: String::from("Guest"),
            last_name: String::from(""),
            id: String::from(""),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Club {
    pub name: String,
    pub id: String,
}

impl Club {
    pub fn empty() -> Club {
        Club {
            name: String::from(""),
            id: String::from(""),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub id: String,
}

impl Team {
    pub fn empty() -> Team {
        Team {
            name: String::from(""),
            id: String::from(""),
        }
    }
}
