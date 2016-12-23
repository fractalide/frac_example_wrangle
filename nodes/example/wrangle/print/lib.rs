#[macro_use]
extern crate rustfbp;
extern crate capnp;

fn print_data(mut msg: rustfbp::ports::Msg)  -> Result<(String,String,String,String)>
{
    let data: quadruple::Reader = try!(msg.read_schema());
    let min = data.get_first().to_string();
    let max =  data.get_second().to_string();
    let average =  data.get_third().to_string();
    let median = data.get_fourth().to_string();
    Ok((min.clone(),max.clone(),average.clone(),median.clone()))
}

agent! {
    input(raw: quadruple, anonymous: quadruple),
    fn run(&mut self) -> Result<Signal> {
        let (min, max, average, median): (String, String, String, String) = try!(print_data(try!(self.input.raw.recv())));
        println!("raw: min: {}, max: {}, average: {}, median: {}", min, max, average, median);
        let (min, max, average, median): (String, String, String, String) = try!(print_data(try!(self.input.anonymous.recv())));
        println!("anonymous: min: {}, max: {}, average: {}, median: {}", min, max, average, median);
        Ok(End)
    }
}
