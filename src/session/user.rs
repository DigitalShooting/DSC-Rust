



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






#[cfg(test)]
mod test {
    use session::user::*;

    #[test]
    fn test_user_empty() {
        let user = User::empty();
        assert_eq!("", user.first_name);
        assert_eq!("", user.last_name);
        assert_eq!("", user.id);
    }

    #[test]
    fn test_user_guest() {
        let user = User::guest();
        assert_eq!("Guest", user.first_name);
        assert_eq!("", user.last_name);
        assert_eq!("", user.id);
    }

    #[test]
    fn test_club_empty() {
        let club = Club::empty();
        assert_eq!("", club.name);
        assert_eq!("", club.id);
    }

    #[test]
    fn test_team_empty() {
        let team = Team::empty();
        assert_eq!("", team.name);
        assert_eq!("", team.id);
    }

}
