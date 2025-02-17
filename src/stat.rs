use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Readlength;
use crate::nanoporepacbio::Sequence;
use std::error::Error;

/*

  Author Gaurav Sablok
  SLB Potsdam
  Date 2025-2-17

*/

pub fn stats(path: &str) -> Result<String, Box<dyn Error>> {
    let sequencevector: Vec<Sequence> = fastareturn(path).unwrap();

    let mut readvec: Vec<Readlength> = Vec::new();
    let mut fivelength = 0usize;
    let mut hundredlength = 0usize;
    let mut hundredfiftylength = 0usize;
    let mut twohundredlength = 0usize;
    let mut morethanlength = 0usize;
    for i in sequencevector.iter() {
        if i.sequence.to_string().len() <= 50000usize {
            fivelength += 1usize;
        } else if i.sequence.to_string().len() > 50000usize
            && i.sequence.to_string().len() <= 100000usize
        {
            hundredlength += 1usize;
        } else if i.sequence.to_string().len() > 100000usize
            && i.sequence.to_string().len() <= 150000usize
        {
            hundredfiftylength += 1usize;
        } else if i.sequence.to_string().len() > 150000usize
            && i.sequence.to_string().len() <= 200000usize
        {
            twohundredlength += 1usize;
        } else if i.sequence.to_string().len() > 200000usize {
            morethanlength += 1usize;
        }
    }
    readvec.push(Readlength {
        five: fivelength,
        hundred: hundredlength,
        hundredfifty: hundredfiftylength,
        twohundred: twohundredlength,
        morethan: morethanlength,
    });

    for i in readvec.iter() {
        println!("Read length greater than 50KB:{}\tRead length greater than 100KB:{}\tRead length greater than 150KB:{}\tRead length greater than 200KB{}\tReads length more than that:{}", i.five, i.hundred, i.hundredfifty, i.twohundred, i.morethan);
    }

    Ok("The result have been printed as follows".to_string())
}
