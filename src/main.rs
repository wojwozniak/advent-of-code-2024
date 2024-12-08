#![allow(dead_code)]

mod problem01;
mod problem02;
mod problem03;
mod problem04;
mod problem05;
mod problem06;
mod problem07;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //problem01::problem01().map_err(|_| "problem01 failed")?;
    //problem02::problem02().map_err(|_| "problem02 failed")?;
    //problem03::problem03().map_err(|_| "problem03 failed")?;
    //problem04::problem04().map_err(|_| "problem04 failed")?;
    //problem05::problem05().map_err(|_| "problem05 failed")?;
    //problem06::problem06().map_err(|_| "problem05 failed")?;
    problem07::problem07().map_err(|_| "problem07 failed")?;
    Ok(())
}
