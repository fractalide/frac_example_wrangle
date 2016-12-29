#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    input(input: ntup_list_triple_ttt),
    output(output: ntup_list_triple_ttt),
    fn run(&mut self) -> Result<Signal> {
        let mut msg = try!(self.input.input.recv());
        let anon_reader: ntup_list_triple_ttt::Reader = try!(msg.read_schema());
        let to_anon_triple = try!(anon_reader.get_list());

        let mut anonymized_bean_counter = HashMap::new();
        let first = to_anon_triple.get(0).get_first()?.get_text()?;
        for i in 0..to_anon_triple.len() {
            let second = to_anon_triple.get(i).get_second()?.get_text()?;
            let third = to_anon_triple.get(i).get_third()?.get_text()?;
            if i32::from_str(third).unwrap() < 6 {
                continue;
            }
            anonymized_bean_counter.insert(second, third);
        }
        let mut fin_msg = Msg::new();
        {
            let ip = fin_msg.build_schema::<ntup_list_triple_ttt::Builder>();
            let mut fin_triple = ip.init_list(anonymized_bean_counter.len() as u32);
            let mut i :u32 = 0;
            for (key,val) in anonymized_bean_counter.iter() {
                fin_triple.borrow().get(i).get_first()?.set_text(first);
                fin_triple.borrow().get(i).get_second()?.set_text(key);
                fin_triple.borrow().get(i).get_third()?.set_text(format!("{}",val).as_str());
                i += 1;
            }
        }
        try!(self.output.output.send(fin_msg));
        Ok(End)
    }
}
