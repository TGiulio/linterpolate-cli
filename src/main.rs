use clap::Parser;
use linterpolate::{
    self, get_intercept, get_line_from_point_and_slope, get_line_from_points, get_slope,
    get_x_from_y, get_y_from_x,
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

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    match cli.element {
        Element::Line => {
            if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
                let line = get_line_from_points(
                    cli.x1.unwrap(),
                    cli.y1.unwrap(),
                    cli.x2.unwrap(),
                    cli.y2.unwrap(),
                );
                if line.is_some() {
                    let line = line.unwrap();
                    println!("the line equation is y = {}x + {}", line.m, line.q);
                } else {
                    println!("couldn't compute a line; please, check your input");
                }
            } else if cli.x1.is_some() && cli.y1.is_some() && cli.slope.is_some() {
                let line = get_line_from_point_and_slope(
                    cli.x1.unwrap(),
                    cli.y1.unwrap(),
                    cli.slope.unwrap(),
                );
                println!("the line equation is y = {}x + {}", line.m, line.q);
            } else {
                println!("couldn't compute a line; please, check your input to specify x1, y1, x2, y2 or x1, y1, and slope");
            }
        }
        Element::Slope => {
            if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
                let slope = get_slope(
                    cli.x1.unwrap(),
                    cli.y1.unwrap(),
                    cli.x2.unwrap(),
                    cli.y2.unwrap(),
                );
                if slope.is_some() {
                    println!("the slope is {}", slope.unwrap())
                } else {
                    println!("couldn't compute a slope; please, check your input");
                }
            } else {
                println!(
                    "couldn't compute a slope; please, check your input to specify x1, y1, x2, y2"
                );
            }
        }
        Element::Intercept => {
            if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
                let line = get_line_from_points(
                    cli.x1.unwrap(),
                    cli.y1.unwrap(),
                    cli.x2.unwrap(),
                    cli.y2.unwrap(),
                );
                if line.is_some() {
                    let line = line.unwrap();
                    println!("the line intercept is {}", line.q);
                } else {
                    println!("couldn't compute a line; please, check your input");
                }
            } else if cli.x1.is_some() && cli.y1.is_some() && cli.slope.is_some() {
                let intercept = get_intercept(cli.x1.unwrap(), cli.y1.unwrap(), cli.slope.unwrap());
                println!("the line intercept is {}", intercept);
            } else {
                println!("couldn't compute the intercept; please, check your input to specify x1, y1, x2, y2 or x1, y1, and slope");
            }
        }
        Element::X => {
            if cli.y.is_some() {
                let y = cli.y.unwrap();
                if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
                    let line = get_line_from_points(
                        cli.x1.unwrap(),
                        cli.y1.unwrap(),
                        cli.x2.unwrap(),
                        cli.y2.unwrap(),
                    );
                    if line.is_some() {
                        let line = line.unwrap();
                        println!("the line equation is y = {}x + {}", line.m, line.q);
                        let x = get_x_from_y(line, y);
                        if x.is_some() {
                            println!("x value when y = {} is ", x.unwrap());
                        } else {
                            println!(
                                "couldn't compute x; please, check your slope to be not equal to 0"
                            );
                        }
                    } else {
                        println!("couldn't compute line; please, check your input");
                    }
                } else if cli.x1.is_some() && cli.y1.is_some() && cli.slope.is_some() {
                    let line = get_line_from_point_and_slope(
                        cli.x1.unwrap(),
                        cli.y1.unwrap(),
                        cli.slope.unwrap(),
                    );
                    println!("the line equation is y = {}x + {}", line.m, line.q);
                    let x = get_x_from_y(line, y);
                    if x.is_some() {
                        println!("x value when y = {} is {}", y, x.unwrap());
                    } else {
                        println!(
                            "couldn't compute x; please, check your slope to be not equal to 0"
                        );
                    }
                } else if cli.slope.is_some() && cli.intercept.is_some() {
                    let line = linterpolate::Line {
                        m: cli.slope.unwrap(),
                        q: cli.intercept.unwrap(),
                    };
                    let x = get_x_from_y(line, y);
                    if x.is_some() {
                        println!("x value when y = {} is {}", y, x.unwrap());
                    } else {
                        println!(
                            "couldn't compute x; please, check your slope to be not equal to 0"
                        );
                    }
                } else {
                    println!("couldn't compute x; please, check your input to always specify Y and then x1, y1, x2, y2 or x1, y1, and slope or slope and intercept");
                }
            } else {
                println!("couldn't compute x; please, check your input to always specify Y");
            }
        }
        Element::Y => {
            if cli.x.is_some() {
                let x = cli.x.unwrap();
                if cli.x1.is_some() && cli.y1.is_some() && cli.x2.is_some() && cli.y2.is_some() {
                    let line = get_line_from_points(
                        cli.x1.unwrap(),
                        cli.y1.unwrap(),
                        cli.x2.unwrap(),
                        cli.y2.unwrap(),
                    );
                    if line.is_some() {
                        let line = line.unwrap();
                        println!("the line equation is y = {}x + {}", line.m, line.q);
                        let y = get_y_from_x(line, x);
                        println!("y value when x = {} is {}", x, y);
                    } else {
                        println!("couldn't compute line; please, check your input");
                    }
                } else if cli.x1.is_some() && cli.y1.is_some() && cli.slope.is_some() {
                    let line = get_line_from_point_and_slope(
                        cli.x1.unwrap(),
                        cli.y1.unwrap(),
                        cli.slope.unwrap(),
                    );
                    println!("the line equation is y = {}x + {}", line.m, line.q);
                    let y = get_y_from_x(line, x);
                    println!("y value when x = {} is {}", x, y);
                } else if cli.slope.is_some() && cli.intercept.is_some() {
                    let line = linterpolate::Line {
                        m: cli.slope.unwrap(),
                        q: cli.intercept.unwrap(),
                    };
                    let y = get_y_from_x(line, x);
                    println!("y value when x = {} is {}", x, y);
                } else {
                    println!("couldn't compute y; please, check your input to always specify X and then x1, y1, x2, y2 or x1, y1, and slope or slope and intercept");
                }
            } else {
                println!("couldn't compute y; please, check your input to always specify X");
            }
        }
        Element::Unrecognized => {
            println!("please insert ONE valid element to find, choosing from: \r 'l' or 'line \r 's' or 'slope' \r 'X' or 'x' \r 'Y' or 'y' \r 'i' or 'intercept' ")
        }
    }
}
