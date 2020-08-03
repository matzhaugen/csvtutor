//tutorial-read-01.rs

use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::vec::Vec;
use std::collections::HashMap;
use nalgebra::base::{DMatrix, Vector};

fn run() -> Result<(), Box<dyn Error>> {
    
    let file = File::open("data/dataSang.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let header = rdr.headers()?.clone();
    println!("{:?}", header);
    println!("Please specify stocks, comma delineated.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("You messed up string typing.");
    let stocks: Vec<&str> = (&*input).split(",").collect();
    let mut trimmed_stocks: Vec<&str> = Vec::new();
    for s in stocks {
        trimmed_stocks.push(s.trim());
    }
    print!("You specified {:?}", trimmed_stocks);
    let mut stock_index = 0;
    let mut stock_indeces = HashMap::new();
    for (i, field) in header.iter().enumerate() {
        println!("{:?} {} vs. {:?}", &i, field, trimmed_stocks);

        if trimmed_stocks.contains(&field) {
            println!("Found it!");
            stock_indeces.insert(field.clone(), i);
        }
    }

    println!("These stocks is in index #{:?}", stock_indeces);
    let mut prices: DMatrix<f32> = DMatrix::zeros(0, stock_indeces.len());
    let mut day_prices: Vector<f32> = Vector::new(stock_indeces.len());
    for (j, result) in rdr.records().enumerate() {
        let record = result?;
        for (stock, index) in stock_indeces.iter() {
            let number = &record[index].parse::<f32>()?; 
            day_prices.push(*number);
            println!("{:?}", day_prices);
        }
        
    //     // println!("{:?}", number);
        prices.insert_row(j, *day_prices);
    }
    // let price_matrix = DMatrix::from_vec(prices.len(), 1, prices);
    println!("{:?}", prices);
    Ok(())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}