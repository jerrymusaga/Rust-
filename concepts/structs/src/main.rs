
struct GrayscalePic {
    pixels: Vec<u8>,
    size: (u32,u32)
}

struct Player {
    name: String,
    age: u8,
    special_skill: Skill,
    fitness: u8,
    jersey_number: (u8,u8)
}

#[derive(Copy, Clone)]
enum Skill{
    Heading,
    Dribbling,
    Goalkeeping
}

fn match_day(p: Player) -> (Player,Player){
    let mut messi = Player{
        age: p.age - 6,
        ..p
    };

    let mut cr7 = Player{
        name: messi.name.clone(),
        ..messi
    };
    messi.name.push_str("amateur");
    cr7.name.push_str("GOAT");
    (messi, cr7)

}

//tuple struct
struct Car(String, u32);

pub struct Queue {
    pub young: Vec<char>,
    pub old: Vec<char>
}

impl Queue {
    fn push(&mut self, c: char){
        self.young.push(c);
    }

    fn pop(&mut self) -> Option<char> {
       if self.old.is_empty(){
           if self.young.is_empty(){
               return None;
           }
           std::mem::swap(&mut self.old, &mut self.young);
           self.old.reverse();
       }
       self.old.pop()
    }
}

fn main() {

    let mut q = Queue {
        young: Vec::new(),
        old: Vec::new(),
    };
    q.push('a');
    q.push('b');
    assert_eq!(q.pop(), Some('a'));
    println!("{:?}",q.old);

    let car_model = Car("Ferrari".to_string(), 2019);
    println!("model:{}, year:{}",car_model.0, car_model.1);

    let champions_league = Player {
        name: "Casillas".to_string(),
        age: 45,
        special_skill: Skill::Goalkeeping,
        fitness: 76,
        jersey_number: (1, 25),
    };
    let (chelsea, porto) = match_day(champions_league);



    let w = 250;
    let h = 450;
    let image = GrayscalePic {
        pixels: vec![3,5,6],
        size: (4, w+h)
    };
    println!("{:?}, {:?}",image.pixels, image.size);
}
