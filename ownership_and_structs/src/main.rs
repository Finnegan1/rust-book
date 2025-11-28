#[derive(Debug)]
struct Rectangel {
    height: f64,
    width: f64,
}

impl Rectangel {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangel) -> bool {
        if self.width > rect.width && self.height > rect.height {
            return true;
        }
        false
    }

    fn square(size: f64) -> Rectangel {
        Rectangel {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangel {
        height: 10.,
        width: 40.,
    };

    let rect2 = Rectangel::square(9.);
    println!("{:?}", rect);
    println!("{:#?}", rect);
    rect.area();

    println!("{}", rect.can_hold(&rect2));
}
