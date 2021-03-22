extern crate structopt;
use structopt::StructOpt;

extern crate anyhow;
use anyhow::Result;

use randcli::eval::eval;
use randcli::parser::parse;

#[derive(StructOpt)]
struct Opt {
    expr: String,

    #[structopt(long)]
    debug: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let expr = parse(opt.expr);
    if opt.debug {
        eprintln!(">>> expr = {:?}", &expr);
    }
    let result = eval(&expr)?;
    println!("{}", result);
    Ok(())
}
