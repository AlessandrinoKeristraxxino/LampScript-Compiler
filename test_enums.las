enum State {
    Playing,
    Paused,
    Stopped,
}

fn main() -> void {
    let s: State = State::Playing;
    
    if s == 0 {
        println?("State is Playing!");
    };
    
    let stopped: State = State::Stopped;
    if stopped == 2 {
        println?("State is Stopped!");
    };
};

{
    main();
};
