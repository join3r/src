use std::io;
//kalkulacka2.0//////////////////////////////
///////////////////////////////////////

#[derive(Copy, Clone)]
struct Vstupy {
    a: f64,
    b: f64,
}
impl Vstupy {
    fn plus(self) -> f64 {
        self.a + self.b
    }
    fn minus(self) -> f64 {
        self.a - self.b
    }
    fn delene(self) -> f64 {
        self.a / self.b
    }
    fn krat(self) -> f64 {
        self.a - self.b
    }
}
fn main() {

  
    println!("daj cislo");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let prvecislo: f64 = input.trim().parse().unwrap();

    
    println!("daj operaciu +-*/");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let operacia = ibaprvy(input, 1);

    println!("daj druhe cislo");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let druhecislo: f64 = input.trim().parse().unwrap();

    let hodnoty = Vstupy {
        a: prvecislo,
        b: druhecislo,
    };

    if operacia == "+" {
        println!(
            " {} {} {} = {}",
            prvecislo,
            operacia,
            druhecislo,
            hodnoty.plus()
        )
    }
    if operacia == "-" {
        println!(
            " {} {} {} = {}",
            prvecislo,
            operacia,
            druhecislo,
            hodnoty.minus()
        )
    }
    if operacia == "/" {
        println!(
            " {} {} {} = {}",
            prvecislo,
            operacia,
            druhecislo,
            hodnoty.delene()
        )
    }
    if operacia == "*" {
        println!(
            " {} {} {} = {}",
            prvecislo,
            operacia,
            druhecislo,
            hodnoty.krat()
        )
    }
  
}
fn ibaprvy(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}
