#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    inarr(input: list_triple),
    output(output: list_triple),
    accumulator(list_triple),
    fn run(&mut self) -> Result<Signal> {
        for ins in self.inarr.input.values()
        {
            println!("Chunk completed!");
            let mut ip = ins.recv()?;
            let chunk_reader: list_triple::Reader = ip.read_schema()?;
            let input_triple = chunk_reader.get_triples()?;

            let mut ip_acc = self.input.accumulator.recv()?;
            let acc_reader: list_triple::Reader = try!(ip_acc.read_schema());
            let acc_triple = acc_reader.get_triples()?;
            let acc_length = acc_triple.len() as u32;
            let input_length = input_triple.len() as u32;

            let mut new_acc_msg = Msg::new();
            {
                let ip = new_acc_msg.build_schema::<list_triple::Builder>();
                let mut new_acc_triple = ip.init_triples(acc_length + input_length);
                let first = input_triple.get(0).get_first()?;
                let mut i :u32 = 0;
                for i in 0..input_triple.len() {
                    let second = input_triple.get(i).get_second()?;
                    let third = input_triple.get(i).get_third()?;
                    new_acc_triple.borrow().get(i).set_first(first);
                    new_acc_triple.borrow().get(i).set_second(second);
                    new_acc_triple.borrow().get(i).set_third(third);
                }
                for i in 0..acc_triple.len() {
                    let second = acc_triple.get(i).get_second()?;
                    let third = acc_triple.get(i).get_third()?;
                    new_acc_triple.borrow().get(i+input_length).set_first(first);
                    new_acc_triple.borrow().get(i+input_length).set_second(second);
                    new_acc_triple.borrow().get(i+input_length).set_third(third);
                }
            }
            self.output.accumulator.send(new_acc_msg)?;
        }

        let mut ip_acc = self.input.accumulator.recv()?;
        let acc_reader: list_triple::Reader = ip_acc.read_schema()?;
        let acc_triple = acc_reader.get_triples()?;

        let mut large_sized_bean_counter = HashMap::new();
        for i in 0..acc_triple.len() {
            let bean = large_sized_bean_counter.entry(try!(acc_triple.get(i).get_second())).or_insert(0);
            *bean += i32::from_str(try!(acc_triple.get(i).get_third())).unwrap();
        }

        let mut fin_msg = Msg::new();
        {
            let ip = fin_msg.build_schema::<list_triple::Builder>();
            let mut fin_triple = ip.init_triples(large_sized_bean_counter.len() as u32);
            let first = try!(acc_triple.get(0).get_first());
            let mut i :u32 = 0;
            for (key,val) in large_sized_bean_counter.iter() {
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
