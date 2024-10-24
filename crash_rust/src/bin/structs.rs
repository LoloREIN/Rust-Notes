struct Redfox {
    enemy: bool,
    life: u32,
}
impl Redfox{
    fn new() -> Self{
        Self{
            enemy: true,
            life: 70,
        }
    }
}

fn main() {

    let mut fox = Redfox::new();
    let life_left = fox.life;
    fox.enemy = false;

    println!(
        "Are you an enemy {} ",fox.enemy);
}