mod owner;
mod pet;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {}

pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/create-owner"]
    CreateOwner,
    #[to = "/app/create-pet/{id}"]
    CreatePet(i32),
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}
