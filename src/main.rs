use colored::Colorize;
extern crate rand;
use rand::Rng;

fn main() {
    println!(
	"{}","Welcome to Roulette!".bold()
    );
    println!(
        "{}","Starting Balance: $500"
    );
    println!(
	"{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
	"0".green(),
	"01".red(),
	"02",
	"03".red(),
	"04",
	"05".red(),
	"06",
	"07".red(),
	"08",
	"09".red(),
	"10",
	"11",
	"12".red(),
	"13",
	"14".red(),
	"15",
	"16".red(),
	"17",
	"18".red()
    );
    println!(
	"{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        "0".green(),
	"19".red(),
        "20",
        "21".red(),
        "22",
        "23".red(),
        "24",
        "25".red(),
        "26",
        "27".red(),
        "28",
        "29",
        "30".red(),
        "31",
        "32".red(),
        "33",
        "34".red(),
        "35",
        "36".red()
    );
    println!(
	"It's time to {}, son!",
	"GAMBLE".bold() 	
    );	
    
    let _black  = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 32, 33, 35];

    let _red  = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];

    let _first = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];

    let _last = [19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36];
    
    

    //fns for payouts, match for logic

    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..36);
    let nprint = n.to_string();

/*  
    if n == (2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35) {
	println!("Rolled : {}", n);
    } else if n == 0 {
	println!("Rolled : {}", nprint.green());
    } else {
	println!("Rolled : {}", nprint.red());
    };
*/

    match n {
        0 => println!("Rolled : {}", nprint.green()),
        1 => println!("Rolled : {}", nprint.red()),
        3 => println!("Rolled : {}", nprint.red()),
        5 => println!("Rolled : {}", nprint.red()),
        7 => println!("Rolled : {}", nprint.red()),
        9 => println!("Rolled : {}", nprint.red()),
        12 => println!("Rolled : {}", nprint.red()),
        14 => println!("Rolled : {}", nprint.red()),
        16 => println!("Rolled : {}", nprint.red()),
        18 => println!("Rolled : {}", nprint.red()),
        19 => println!("Rolled : {}", nprint.red()),
        21 => println!("Rolled : {}", nprint.red()),
        23 => println!("Rolled : {}", nprint.red()),
        25 => println!("Rolled : {}", nprint.red()),
        27 => println!("Rolled : {}", nprint.red()),
        30 => println!("Rolled : {}", nprint.red()),
        32 => println!("Rolled : {}", nprint.red()),
	34 => println!("Rolled : {}", nprint.red()),
        36 => println!("Rolled : {}", nprint.red()),
        _ => println!("Rolled : {}", n),
    }

    //match can be more verbose than if when it comes to simple things like printing, but its also more readable.
    //for a program like this match would be better for most of the logic.
}
