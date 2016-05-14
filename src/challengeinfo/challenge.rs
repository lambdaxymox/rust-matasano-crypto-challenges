pub struct ChallengeInfo<'a> {
    pub set_number: usize,
    pub challenge_number: usize,
    pub title: &'a str,
    pub description: &'a str,
    pub url: &'a str,
}

pub struct Challenge<'a> {
    pub info: ChallengeInfo<'a>,
    pub func: fn() -> String,
}

