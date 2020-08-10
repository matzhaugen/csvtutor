//tutorial-read-01.rs

use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::vec::Vec;
use std::collections::HashMap;

fn average(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
}

fn get_returns_mut(prices: Vec<Vec<f32>>, n: usize, p: usize) -> Vec<Vec<f32>> {
    let mut returns = vec![vec![0.0; n]; p];
    for j in 0..p {
        for i in 1..n {
            returns[j][i] = (prices[j][i] - prices[j][i-1]) / prices[j][i-1];
        }
    }
    returns
}

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
    
    let mut stock_indeces = HashMap::new();
    for (i, field) in header.iter().enumerate() {
        println!("{:?} {} vs. {:?}", &i, field, trimmed_stocks);

        if trimmed_stocks.contains(&field) {
            println!("Found it!");
            stock_indeces.insert(field.clone(), i);
        }
    }
    let n_stocks = stock_indeces.len();

    println!("These stocks are in index #{:?}", stock_indeces);
    
    let mut daily: Vec<Vec<f32>> = vec![Vec::new(); n_stocks];
    let mut n_samples = 0;
    for (j, result) in rdr.records().enumerate() {
        let record = result?;
        for (i, (_, &index)) in stock_indeces.iter().enumerate() {
            let number = &record[index].parse::<f32>()?; 
            daily[i].push(*number);
        }
        n_samples = j;
    } 
    n_samples += 1;  
    let returns = get_returns_mut(daily, n_samples, n_stocks) ;       
    let mut avg = vec![0.0; n_stocks];
    let mut centered = vec![vec![0.0; n_samples - 1]; n_stocks];
    
    for i in 0..n_stocks {
        avg[i] = average(&returns[i]);
    }
    for i in 0..n_stocks {
        for j in 0..n_samples - 1 {
            centered[i][j] = returns[i][j] - avg[i]    
        }
    }
    let mut cov = vec![vec![0.0; n_stocks]; n_stocks];

    for i in 0..n_stocks {
        for j in i..n_stocks {
            for k in 0..n_samples-1 {
                cov[i][j] += centered[i][k] * centered[j][k]
            }
            cov[i][j] = cov[i][j] / (n_samples-1) as f32 
        }
    }

    // get_weights(dates, prices);

    println!("samples: {:?}, stocks: {}, avg: {:?}", n_samples, n_stocks, avg);


    println!("covariance: {:?}", cov);
    Ok(())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}