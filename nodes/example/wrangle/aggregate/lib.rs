#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    inarr(input: list_ntuple_triple_ttt),
    output(output: list_ntuple_triple_ttt),
    accumulator(list_ntuple_triple_ttt),
    fn run(&mut self) -> Result<Signal> {
        for ins in self.inarr.input.values()
        {
            println!("Chunk completed!");
            let mut ip = ins.recv()?;
            let list_triple_reader: list_ntuple_triple_ttt::Reader = ip.read_schema()?;
            let list_triple = list_triple_reader.get_list()?;

            let mut ip_acc = self.input.accumulator.recv()?;
            let acc_reader: list_ntuple_triple_ttt::Reader = try!(ip_acc.read_schema());
            let acc_triple = acc_reader.get_list()?;
            let acc_length = acc_triple.len() as u32;
            let input_length = list_triple.len() as u32;

            let mut new_acc_msg = Msg::new();
            {
                let ip = new_acc_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
                let mut new_acc_triple = ip.init_list(acc_length + input_length);
                let mut i :u32 = 0;
                for i in 0..list_triple.len() {
                    let first = list_triple.get(i).get_first()?.get_text();
                    let second = list_triple.get(i).get_second()?.get_text();
                    let third = list_triple.get(i).get_third()?.get_text();
                    new_acc_triple.borrow().get(i).get_first()?.set_text(first?);
                    new_acc_triple.borrow().get(i).get_second()?.set_text(second?);
                    new_acc_triple.borrow().get(i).get_third()?.set_text(third?);
                }
                for i in 0..acc_triple.len() {
                    let first = list_triple.get(i).get_first()?.get_text();
                    let second = acc_triple.get(i).get_second()?.get_text();
                    let third = acc_triple.get(i).get_third()?.get_text();
                    new_acc_triple.borrow().get(i+input_length).get_first()?.set_text(first?);
                    new_acc_triple.borrow().get(i+input_length).get_second()?.set_text(second?);
                    new_acc_triple.borrow().get(i+input_length).get_third()?.set_text(third?);
                }
            }
            self.output.accumulator.send(new_acc_msg)?;
        }

        let mut ip_acc = self.input.accumulator.recv()?;
        let acc_reader: list_ntuple_triple_ttt::Reader = ip_acc.read_schema()?;
        let acc_triple = acc_reader.get_list()?;

        let mut large_sized_bean_counter = HashMap::new();
        for i in 0..acc_triple.len() {
            let bean = large_sized_bean_counter.entry(try!(acc_triple.get(i).get_second()?.get_text())).or_insert(0);
            *bean += i32::from_str(try!(acc_triple.get(i).get_third()?.get_text())).unwrap();
        }

        let mut fin_msg = Msg::new();
        {
            let ip = fin_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
            let mut fin_triple = ip.init_list(large_sized_bean_counter.len() as u32);
            let first = try!(acc_triple.get(0).get_first()?.get_text());
            let mut i :u32 = 0;
            for (key,val) in large_sized_bean_counter.iter() {
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
