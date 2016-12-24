#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    input(input: list_ntuple_triple_ttt),
    output(output: list_ntuple_triple_ttt, next : prim_text),
    accumulator(list_ntuple_triple_ttt),
    fn run(&mut self) -> Result<Signal> {
        loop{
            let mut msg = self.input.input.recv()?;
            let input_triple: list_ntuple_triple_ttt::Reader = msg.read_schema()?;
            let input_triple = input_triple.get_list()?;
            if input_triple.get(0).get_first()?.get_text()? == "end" {
                let mut acc_msg = self.input.accumulator.recv()?;
                let acc_reader: list_ntuple_triple_ttt::Reader = acc_msg.read_schema()?;
                let acc_triple = acc_reader.get_list()?;
                let mut feedback_ip = Msg::new();
                {
                    let ip = feedback_ip.build_schema::<list_ntuple_triple_ttt::Builder>();
                    let mut feedback_list = ip.init_list(acc_triple.len() as u32);
                    let first = acc_triple.get(0).get_first()?.get_text()?;
                    for i in 0..feedback_list.len() {
                        feedback_list.borrow().get(i).get_first()?.set_text(first);
                        feedback_list.borrow().get(i).get_second()?.set_text(acc_triple.get(i).get_second()?.get_text()?);
                        feedback_list.borrow().get(i).get_third()?.set_text(format!("{}", acc_triple.get(i).get_third()?.get_text()?).as_str());
                    }
                }
                self.output.output.send(feedback_ip)?;
                break;
            } else {
                let mut acc_msg = self.input.accumulator.recv()?;
                let acc_reader: list_ntuple_triple_ttt::Reader = acc_msg.read_schema()?;
                let acc_triple = acc_reader.get_list()?;
                let acc_length = acc_triple.len() as u32;
                let input_length = input_triple.len() as u32;
                if acc_length == 0 {
                    let mut acc_msg = Msg::new();
                    {
                        let ip = acc_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
                        let mut acc_triple = ip.init_list(input_length);
                        for i in 0..input_triple.len() {
                            acc_triple.borrow().get(i).get_first()?.set_text(input_triple.get(i).get_first()?.get_text()?);
                            acc_triple.borrow().get(i).get_second()?.set_text(input_triple.get(i).get_second()?.get_text()?);
                            acc_triple.borrow().get(i).get_third()?.set_text(input_triple.get(i).get_third()?.get_text()?);
                        }
                    }
                    self.output.accumulator.send(acc_msg)?;
                }else {
                    let mut medium_sized_bean_counter = HashMap::new();
                    for i in 0..input_triple.len() {
                        let first = input_triple.get(i).get_first()?.get_text()?;
                        let second = input_triple.get(i).get_second()?.get_text()?;
                        let third = input_triple.get(i).get_third()?.get_text()?;
                        if second.is_empty() || second == "0" {
                            continue;
                        } else {
                            let bean = medium_sized_bean_counter.entry(second).or_insert(0);
                            *bean += i32::from_str(third).unwrap();
                        }
                    }
                    for i in 0..acc_triple.len() {
                        let first = acc_triple.get(i).get_first()?.get_text()?;
                        let second = acc_triple.get(i).get_second()?.get_text()?;
                        let third = acc_triple.get(i).get_third()?.get_text()?;
                        if second.is_empty() || second == "0" {
                            continue;
                        } else {
                            let bean = medium_sized_bean_counter.entry(second).or_insert(0);
                            *bean += i32::from_str(third).unwrap();
                        }
                    }
                    let mut new_acc_msg = Msg::new();
                    {
                        let ip = new_acc_msg.build_schema::<list_ntuple_triple_ttt::Builder>();
                        let mut new_acc_triple = ip.init_list(medium_sized_bean_counter.len() as u32);
                        let first = acc_triple.get(0).get_first()?.get_text()?;
                        let mut i :u32 = 0;
                        for (key,val) in medium_sized_bean_counter.iter() {
                            new_acc_triple.borrow().get(i).get_first()?.set_text(first);
                            new_acc_triple.borrow().get(i).get_second()?.set_text(key);
                            new_acc_triple.borrow().get(i).get_third()?.set_text(format!("{}",val).as_str());
                            i += 1;
                        }
                    }
                    self.output.accumulator.send(new_acc_msg)?;
                }
            }
            let mut next_msg = Msg::new();
            {
                let mut ip = next_msg.build_schema::<prim_text::Builder>();
                ip.set_text("next");
            }
            self.output.next.send(next_msg)?;
        }
        Ok(End)
    }
}
