mod generator;
use std::io;
use std::io::Write;
use generator::generate;

fn main() {
    let mut data = String::new();
    println!("What is the text you would like to encode?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data).unwrap();
    let data = data.trim(); 

    println!("{}", data);

    loop{
        let mut mode = String::new();
        println!("What level of error correction would you like?");
        println!("1. Level L: Recovers 7% of data");
        println!("2. Level M: Recovers 15% of data");
        println!("3. Level Q: Recovers 25% of data");
        println!("4. Level H: Recovers 30% of data");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut mode).unwrap();
        let mode = mode.trim();

        match mode{
            "1" => {
                if data.len() > 77 {
                    println!("Data is too big to be encoded!!");
                    break;
                }
                else {
                    generate(data); 
                    break;
                }
            },
            "2" => {
                if data.len() <= 61 {
                    println!("Generating using M");
                    break;
                } else {
                    println!("Too big for this mode! Try another!");
                }
            },
            "3" => {
                if data.len() <= 47 {
                    println!("Generating using Q");
                    break;
                } else {
                    println!("Too big for this mode! Try another!");
                }
            },
            "4" => {
                if data.len() <= 35 {
                    println!("Generating using H");
                    break;
                } else {
                    println!("Too big for this mode! Try another!");
                }
            },
            _ => println!("Invalid Option!")
        };
    }
}
