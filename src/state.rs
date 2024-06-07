pub trait State {
    fn new() {}

    fn exit() {}

    fn update() {}

    fn physics_update() {}
}
