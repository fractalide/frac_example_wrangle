#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    input(input: list_triple),
    output(output: list_triple),
    fn run(&mut self) -> Result<Signal> {
        let mut msg = try!(self.input.input.recv());
        let anon_reader: list_triple::Reader = try!(msg.read_schema());
        let to_anon_triple = try!(anon_reader.get_triples());

        let mut anonymized_bean_counter = HashMap::new();
        let first = try!(to_anon_triple.get(0).get_first());
        for i in 0..to_anon_triple.len() {
            let second = try!(to_anon_triple.get(i).get_second());
            let third = try!(to_anon_triple.get(i).get_third());
            if i32::from_str(third).unwrap() < 6 {
                continue;
            }
            anonymized_bean_counter.insert(second, third);
        }
        let mut fin_msg = Msg::new();
        {
            let ip = fin_msg.build_schema::<list_triple::Builder>();
            let mut fin_triple = ip.init_triples(anonymized_bean_counter.len() as u32);
            let mut i :u32 = 0;
            for (key,val) in anonymized_bean_counter.iter() {
                fin_triple.borrow().get(i).set_first(first);
                fin_triple.borrow().get(i).set_second(key);
                fin_triple.borrow().get(i).set_third(format!("{}",val).as_str());
                i += 1;
            }
        }
        try!(self.output.output.send(fin_msg));
        Ok(End)
    }
}
