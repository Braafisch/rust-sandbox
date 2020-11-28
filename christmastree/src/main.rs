use clap::{App, Arg};

struct ChristmasTree {
    height: u32,
    has_star: bool,
}

impl ChristmasTree {
    fn draw(&self) {
        if self.has_star {
            self.draw_star();
        }
        self.draw_tree();
        self.draw_trunk();
    }

    fn draw_star(&self) {
        self.output("*");
    }

    fn draw_tree(&self) {
        for i in 0..self.height {
            let level_width = (i * 2 + 1) as usize;
            self.output(&std::iter::repeat("X").take(level_width).collect::<String>());
        }
    }

    fn draw_trunk(&self) {
        self.output("I")
    }

    fn output(&self, a: &str) {
        println!("{}", format!("{: ^1$}", a, self.width() as usize));
    }

    fn width(&self) -> u32 {
        self.height * 2 - 1
    }
}

fn main() {
    let matches = App::new("christmastree")
        .version("0.1.0")
        .author("wolfi")
        .about("prints a pretty christmas tree (merry christmas!)")
        .arg(
            Arg::with_name("withStar")
                .short("w")
                .long("--withStar")
                .help("add a little start on-top"),
        )
        .arg(
            Arg::with_name("height")
                .value_name("HEIGHT")
                .index(1)
                .required(true)
                .help("height of the tree"),
        )
        .get_matches();

    let tree = ChristmasTree {
        height: matches
            .value_of("height")
            .expect("Missing argument HEIGHT")
            .parse::<u32>()
            .expect("Unable to parse integer"),
        has_star: matches.is_present("withStar"),
    };

    tree.draw();
}
