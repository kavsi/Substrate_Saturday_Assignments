// Fully Qualified Syntax problem solution
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;
    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");  // the blank is filled with 'Pilot::fly(&person)'
    assert_eq!(Wizard::fly(&person), "Up!");    // the blank is filled with 'Wizard::fly(&person)'

    assert_eq!(person.fly(), "*waving arms furiously*");   // the blank is filled with 'person.fly()'

    println!("Success!")
}