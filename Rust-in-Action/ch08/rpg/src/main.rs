use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);

        print!("{:?}が呪文を呟く。　", self);
        if spell_is_successful {
            println!("{:?}は明るく光る", thing);
        } else {
            println!("{:?}はシューっといって、つまらないものに変わる", thing);
            *thing = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5 //Dwarf は下手な魔法使い
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95 //Elfの魔法は滅多に失敗しない
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8 //Humanは上手だ。そう間違わない
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    // それぞれ違う型だが、Enchanterトレイトを実装するので、同じVecに入れられる
    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it)
}
