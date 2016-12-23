#[macro_use]
extern crate rustfbp;
extern crate capnp;

agent! {
    input(input: list_tuple),
    output(output: list_triple),
    option(value_string),
    fn run(&mut self) -> Result<Signal> {
        let mut opt = self.recv_option();
        let extract_key: value_string::Reader = opt.read_schema()?;

        let mut ip = try!(self.input.input.recv());
        let list_tuple: list_tuple::Reader = try!(ip.read_schema());
        let list_tuple = try!(list_tuple.get_tuples());

        if try!(list_tuple.get(0).get_first()) != "end" {
            let mut small_sized_bean_counter = HashMap::new();
            for i in 0..list_tuple.len()
            {
                if try!(list_tuple.get(i).get_first()) == try!(extract_key.get_value()) {
                    let bean = small_sized_bean_counter.entry(try!(list_tuple.get(i).get_second())).or_insert(0);
                    *bean += 1;
                }
            }
            if small_sized_bean_counter.len() == 0 {
                small_sized_bean_counter.insert("0",0);
            }
            let mut new_msg = Msg::new();
            {
                let ip = new_msg.build_schema::<list_triple::Builder>();
                let mut triples = ip.init_triples(small_sized_bean_counter.len() as u32);
                let mut i: u32 = 0;
                for (key, val) in small_sized_bean_counter.iter() {
                    triples.borrow().get(i).set_first(try!(extract_key.get_value()));
                    triples.borrow().get(i).set_second(format!("{}",key).as_str());
                    triples.borrow().get(i).set_third(format!("{}",val).as_str());
                    i += 1;
                }
            }
            self.output.output.send(new_msg)?;
        } else {
            let mut end_msg = Msg::new();
            {
                let ip = end_msg.build_schema::<list_triple::Builder>();
                let mut triples = ip.init_triples(1);
                triples.borrow().get(0).set_first("end");
            }
            self.output.output.send(end_msg)?;
        }
        Ok(End)
    }
}
