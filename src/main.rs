use std::{error::Error, fs};
// use pest::prec_climber::{PrecClimber, Assoc, Operator};
use pest::Parser;
use pest_derive::Parser;
use tera::{Context, Tera};

#[derive(Parser)]
#[grammar = "../csv.pest"]
pub struct CSVParser;

fn main() -> Result<(), Box<dyn Error>> {
    let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    println!("pest: {:?}", successful_parse);

    let unparsed = fs::read_to_string("sample.csv")?;
    let file = CSVParser::parse(Rule::file, &unparsed)?.next().unwrap();
    // println!("tokens: {:?}", file.clone().tokens());
    let mut count = 0;
    let records: Vec<Vec<_>> = file
        .into_inner()
        .map(|record| match record.as_rule() {
            Rule::record => {
                count += 1;
                record
                    .into_inner()
                    .map(|field| field.as_str().parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
            }
            Rule::EOI => [].into(),
            _ => unreachable!(),
        })
        .collect();
    println!("records: {records:?}");

    let tera = Tera::new("templates/*.html")?;
    let mut context = Context::new();
    context.insert("value", &20);
    let rendered = tera.render("hello.html", &context)?;
    println!("tera: {rendered}");
    Ok(())
}
