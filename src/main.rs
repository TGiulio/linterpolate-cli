use clap::Parser;
use linterpolate::{
    self, get_line_from_point_and_slope, get_line_from_points, get_x_from_y, get_y_from_x,
};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'f', long = "find")]
    element: Element,

    #[arg(long, allow_hyphen_values = true)]
    x1: Option<f64>,
    #[arg(long, allow_hyphen_values = true)]
    y1: Option<f64>,

    #[arg(long, allow_hyphen_values = true)]
    x2: Option<f64>,
    #[arg(long, allow_hyphen_values = true)]
    y2: Option<f64>,

    #[arg(short = 'x', allow_hyphen_values = true)]
    x: Option<f64>,
    #[arg(short = 'y', allow_hyphen_values = true)]
    y: Option<f64>,

    #[arg(short, long, allow_hyphen_values = true)]
    slope: Option<f64>,

    #[arg(short, long, allow_hyphen_values = true)]
    intercept: Option<f64>,
}

#[derive(Debug, Clone)]
enum Element {
    Line,
    X,
    Y,
    Slope,
    Intercept,
    Unrecognized,
}

impl From<String> for Element {
    fn from(str: String) -> Element {
        match str.as_str() {
            "l" => Element::Line,
            "line" => Element::Line,
            "s" => Element::Slope,
            "slope" => Element::Slope,
            "i" => Element::Intercept,
            "intercept" => Element::Intercept,
            "x" => Element::X,
            "X" => Element::X,
            "y" => Element::Y,
            "Y" => Element::Y,
            _ => Element::Unrecognized,
        }
    }
}

fn get_line_if_possible(cli: &Cli) -> Option<linterpolate::Line> {
    if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
        let line = get_line_from_points(
            cli.x1.unwrap(),
            cli.y1.unwrap(),
            cli.x2.unwrap(),
            cli.y2.unwrap(),
        );
        if line.is_some() {
            Some(line.unwrap())
        } else {
            None
        }
    } else if cli.x1.is_some() && cli.y1.is_some() && cli.slope.is_some() {
        Some(get_line_from_point_and_slope(
            cli.x1.unwrap(),
            cli.y1.unwrap(),
            cli.slope.unwrap(),
        ))
    } else if cli.slope.is_some() && cli.intercept.is_some() {
        Some(linterpolate::Line {
            m: cli.slope.unwrap(),
            q: cli.intercept.unwrap(),
        })
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();

    let line = get_line_if_possible(&cli);

    if line.is_some() {
        let line = line.unwrap();
        match cli.element {
            Element::Line => {
                println!("the line equation is y = {}x + {}", line.m, line.q);
            }
            Element::Slope => {
                println!("the slope is {}", line.m)
            }
            Element::Intercept => {
                println!("the line intercept is {}", line.q);
            }
            Element::X => {
                if cli.y.is_some() {
                    let y = cli.y.unwrap();
                    let x = get_x_from_y(line, y);
                    if x.is_some() {
                        println!("x value when y = {} is {}", y, x.unwrap());
                    } else {
                        println!(
                            "couldn't compute x; please, check your slope to be not equal to 0"
                        );
                    }
                }
            }
            Element::Y => {
                if cli.x.is_some() {
                    let x = cli.x.unwrap();
                    let y = get_y_from_x(line, x);
                    println!("y value when x = {} is {}", x, y);
                }
            }
            Element::Unrecognized => {
                println!("please insert ONE valid element to find, choosing from: \n 'l' or 'line \n 's' or 'slope' \n 'X' or 'x' \n 'Y' or 'y' \n 'i' or 'intercept' ")
            }
        }
    }
}
