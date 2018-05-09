

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    user: User,
    club: Club,
    team: Team,
    line: Line,

}

impl Info {
    pub fn new(line: Line) -> Info {
        Info {
            user: User::guest(),
            club: Club::empty(),
            team: Team::empty(),
            line,
        }
    }
}




#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
    last_name: String,
    id: String,
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
struct Club {
    name: String,
    id: String,
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
struct Team {
    name: String,
    id: String,
}

impl Team {
    pub fn empty() -> Team {
        Team {
            name: String::from(""),
            id: String::from(""),
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub short_name: String,
}
impl Line {
    pub fn demo() -> Line {
        Line {
            id: "id0".to_string(),
            name: "Line 1".to_string(),
            short_name: "1".to_string(),
        }
    }
}





#[cfg(test)]
mod test {
    use session::info::*;

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
