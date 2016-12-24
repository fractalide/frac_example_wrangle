#[macro_use]
extern crate rustfbp;
extern crate capnp;

agent! {
    input(input: list_ntuple_tuple_tt),
    output(output: list_ntuple_triple_ttt),
    option(prim_text),
    fn run(&mut self) -> Result<Signal> {
        let mut opt = self.recv_option();
        let extract_key: prim_text::Reader = opt.read_schema()?;

        let mut ip = self.input.input.recv()?;
        let list_tuple: list_ntuple_tuple_tt::Reader = ip.read_schema()?;
        let list_tuple = list_tuple.get_list()?;

        if list_tuple.get(0).get_first()?.get_text()? != "end" {
            let mut small_sized_bean_counter = HashMap::new();
            for i in 0..list_tuple.len()
            {
                if list_tuple.get(i).get_first()?.get_text()? == extract_key.get_text()? {
                    let bean = small_sized_bean_counter.entry(list_tuple.get(i).get_second()?.get_text()?).or_insert(0);
                    *bean += 1;
                }
            }
            if small_sized_bean_counter.len() == 0 {
                small_sized_bean_counter.insert("0",0);
            }
            let mut new_msg = Msg::new();
            {
                let ip = new_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
                let mut triples = ip.init_list(small_sized_bean_counter.len() as u32);
                let mut i: u32 = 0;
                for (key, val) in small_sized_bean_counter.iter() {
                    triples.borrow().get(i).get_first()?.set_text(try!(extract_key.get_text()));
                    triples.borrow().get(i).get_second()?.set_text(format!("{}",key).as_str());
                    triples.borrow().get(i).get_third()?.set_text(format!("{}",val).as_str());
                    i += 1;
                }
            }
            self.output.output.send(new_msg)?;
        } else {
            let mut end_msg = Msg::new();
            {
                let ip = end_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
                let mut triples = ip.init_list(1);
                triples.borrow().get(0).get_first()?.set_text("end");
            }
            self.output.output.send(end_msg)?;
        }
        Ok(End)
    }
}
