// ANCHOR: here
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("這裡是艦長發言。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("起！");
    }
}

impl Human {
    fn fly(&self) {
        println!("*狂揮雙臂*");
    }
}
// ANCHOR_END: here

fn main() {}
