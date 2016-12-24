#[macro_use]
extern crate rustfbp;
extern crate capnp;

fn print_data(mut msg: rustfbp::ports::Msg)  -> Result<(String,String,String,String)>
{
    let data: ntuple_quadruple_u32u32u32f32::Reader = msg.read_schema()?;
    let min = data.get_first()?.get_u32().to_string();
    let max =  data.get_second()?.get_u32().to_string();
    let average =  data.get_third()?.get_u32().to_string();
    let median = data.get_fourth()?.get_f32().to_string();
    Ok((min.clone(),max.clone(),average.clone(),median.clone()))
}

agent! {
    input(raw: ntuple_quadruple_u32u32u32f32, anonymous: ntuple_quadruple_u32u32u32f32),
    fn run(&mut self) -> Result<Signal> {
        let (min, max, average, median): (String, String, String, String) = try!(print_data(try!(self.input.raw.recv())));
        println!("raw: min: {}, max: {}, average: {}, median: {}", min, max, average, median);
        let (min, max, average, median): (String, String, String, String) = try!(print_data(try!(self.input.anonymous.recv())));
        println!("anonymous: min: {}, max: {}, average: {}, median: {}", min, max, average, median);
        Ok(End)
    }
}
