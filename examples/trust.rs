use trust::{
    CheatingAgent, CooperatingAgent, CopycatAgent, DetectiveAgent, Game, GrudgerAgent, RoundOutcome,
};

fn main() {
    let mut game1 = Game::new(
        Box::new(CooperatingAgent::default()),
        Box::new(CheatingAgent::default()),
    );

    let res = game1.play_round();

    println!("{:?}", res);
    println!("{}", game1.left_score());
    println!("{}", game1.right_score());

    let mut game2 = Game::new(
        Box::new(CopycatAgent::default()),
        Box::new(GrudgerAgent::default()),
    );

    let res = game2.play_round();

    println!("{:?}", res);
    println!("{}", game2.left_score());
    println!("{}", game2.right_score());

    let mut game3 = Game::new(
        Box::new(DetectiveAgent::default()),
        Box::new(DetectiveAgent::default()),
    );

    let res = game3.play_round();

    println!("{:?}", res);
    println!("{}", game3.left_score());
    println!("{}", game3.right_score());
}
