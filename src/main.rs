use std::io;

const CHOICES: [(&str, &str);3] = [
    ("shell", "zsh"),
    ("sway", "sway"),
    ("gnome", "startx"),
];

fn print_list() {
    println!("What to launch??\n");

    for (i, (name, _cmd)) in CHOICES.iter().enumerate() {
        println!("{}. {}", i, name);
    }
}

fn get_choice() -> (&'static str, &'static str) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let choice: u8 = buf.trim().parse().expect("not a number");
    CHOICES[choice as usize]
}

fn main() {
    print_list();
    let (name, cmd) = get_choice();
    println!("{}:>>> {}", name, cmd);
}
