enum Result {
    win = 3,
    loss = 0,
    draw = 1,
}

struct Team {
    Name: String,
    MP: u8,
    W: u8,
    D: u8,
    L: u8,
    P: u8,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            Name: name.to_string(),
            MP: 0,
            W: 0,
            D: 0,
            L: 0,
            P: 0,
        }
    }

    pub fn win(&mut self) {
        self.MP += 1;
        self.W += 1;
        self.P += 3;
    }

    pub fn loss(&mut self) {
        self.MP += 1;
        self.L += 1;
    }

    pub fn draw(&mut self) {
        self.MP += 1;
        self.D += 1;
        self.P += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut vecTeam: Vec<Team> = Vec::new();

    let vecline: Vec<&str> = match_results.lines().collect();

    for line in vecline {
        let game: Vec<&str> = line.split(";").collect();

        if vecTeam.iter().find(|team| team.Name == game[0]).is_none() {
            vecTeam.push(Team::new(game[0]));
        }

        if vecTeam.iter().find(|team| team.Name == game[1]).is_none() {
            vecTeam.push(Team::new(game[1]));
        }

        if game[2] == "win" {
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[0]) {
                team.win();
            }
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[1]) {
                team.loss();
            }
        }

        if game[2] == "loss" {
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[0]) {
                team.loss();
            }
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[1]) {
                team.win();
            }
        }

        if game[2] == "draw" {
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[0]) {
                team.draw();
            }
            if let Some(team) = vecTeam.iter_mut().find(|team| team.Name == game[1]) {
                team.draw();
            }
        }
    }

    vecTeam.sort_by(|team1, team2| {
        team2
            .P
            .cmp(&team1.P)
            .then_with(|| team1.Name.cmp(&team2.Name))
    });

    pub fn add_divider(string: &mut String) {
        string.push_str(" |  ");
    }

    let mut output: String =
        "Team                           | MP |  W |  D |  L |  P\n".to_string();

    for team in &vecTeam {
        output.push_str(&format!("{:<30}", team.Name));
        add_divider(&mut output);
        output.push_str(&team.MP.to_string());
        add_divider(&mut output);
        output.push_str(&team.W.to_string());
        add_divider(&mut output);
        output.push_str(&team.D.to_string());
        add_divider(&mut output);
        output.push_str(&team.L.to_string());
        add_divider(&mut output);
        output.push_str(&team.P.to_string());
        output.push_str("\n");
    }
    output.trim_end().to_string()
}
