enum VektorComparison {
    Bigger,
    Smaller,
    Same,
}

struct Vektor {
    x: i32,
    y: i32,
}

struct Vektor3D {
    x: i32,
    y: i32,
    z: i32,
}

// trait to ze se maji velikost, implementuje pro 2d i 3d
trait Size {
    fn size(&self) -> f64;
}

// trait to ze se daji vektory porovnavat, implementuje pro 2d i 3d
trait Comparable {
    fn is_bigger<T: Size>(&self, other: T) -> VektorComparison;
}

// staticka metoda pro vytvareni novych vektoru
impl Vektor {
    fn new(x: i32, y: i32) -> Vektor {
        Vektor { x, y }
    }
}

// staticka metoda pro nove 3d vektory
impl Vektor3D {
    fn new(x: i32, y: i32, z: i32) -> Vektor3D {
        Vektor3D { x, y, z }
    }
}

// instancni metody na zjisteni velikosti a porovnavani velikosti
impl Size for Vektor {
    fn size(&self) -> f64 {
        let before_square_root: i32 = (self.x * self.x + self.y * self.y).abs();
        let square_rooted: f64 = (before_square_root as f64).sqrt();

        square_rooted
    }
}

impl Comparable for Vektor {
    fn is_bigger<Vektor: Size>(&self, other: Vektor) -> VektorComparison {
        let self_size = self.size();
        let other_size = other.size();

        if self_size == other_size {
            VektorComparison::Same
        } else if self_size > other_size {
            VektorComparison::Bigger
        } else {
            VektorComparison::Smaller
        }
    }
}

// instancni metody na zjisteni velikosti a porovnavani velikosti pro 3d vektory
impl Size for Vektor3D {
    fn size(&self) -> f64 {
        let before_square_root: i32 = (self.x * self.x + self.y * self.y + self.z * self.z).abs();
        let square_rooted: f64 = (before_square_root as f64).sqrt();

        square_rooted
    }
}

impl Comparable for Vektor3D {
    fn is_bigger<Vektor3D: Size>(&self, other: Vektor3D) -> VektorComparison {
        let self_size = self.size();
        let other_size = other.size();

        if self_size == other_size {
            VektorComparison::Same
        } else if self_size > other_size {
            VektorComparison::Bigger
        } else {
            VektorComparison::Smaller
        }
    }
}

fn main() {
    let vektor1: Vektor = Vektor::new(5, 4);
    let vektor2: Vektor = Vektor::new(10, 2);

    comparison_printing(vektor1, vektor2);

    let vektor3d1: Vektor3D = Vektor3D::new(5, 6, 7);
    let vektor3d2: Vektor3D = Vektor3D::new(7, 6, 5);

    comparison_printing(vektor3d1, vektor3d2);
}

// 2 argumenty jako vektory, porovna jejich velikost a podle toho printne porovnani, funguje na 3d i 2d pomoci traitu
fn comparison_printing(vektor1: impl Comparable + Size, vektor2: impl Comparable + Size) -> () {
    let comparison: VektorComparison = vektor1.is_bigger(vektor2);

    match comparison {
        VektorComparison::Smaller => println!("vektor1 is smaller vektor2"),
        VektorComparison::Same => println!("the vektors are the same sizes"),
        VektorComparison::Bigger => println!("vektor1 is bigger than vektor2"),
    }
}
