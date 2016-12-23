#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

agent! {
    input(input: list_triple),
    output(output: list_triple, next : value_string),
    accumulator(list_triple),
    fn run(&mut self) -> Result<Signal> {
        loop{
            let mut msg = self.input.input.recv()?;
            let input_triple: list_triple::Reader = msg.read_schema()?;
            let input_triple = input_triple.get_triples()?;
            if input_triple.get(0).get_first()? == "end" {
                let mut acc_msg = self.input.accumulator.recv()?;
                let acc_reader: list_triple::Reader = acc_msg.read_schema()?;
                let acc_triple = acc_reader.get_triples()?;
                let mut feedback_ip = Msg::new();
                {
                    let ip = feedback_ip.build_schema::<list_triple::Builder>();
                    let mut feedback_list = ip.init_triples(acc_triple.len() as u32);
                    let first = acc_triple.get(0).get_first()?;
                    for i in 0..feedback_list.len() {
                        feedback_list.borrow().get(i).set_first(first);
                        feedback_list.borrow().get(i).set_second(acc_triple.get(i).get_second()?);
                        feedback_list.borrow().get(i).set_third(format!("{}", acc_triple.get(i).get_third()?).as_str());
                    }
                }
                self.output.output.send(feedback_ip)?;
                break;
            } else {
                let mut acc_msg = self.input.accumulator.recv()?;
                let acc_reader: list_triple::Reader = acc_msg.read_schema()?;
                let acc_triple = acc_reader.get_triples()?;
                let acc_length = acc_triple.len() as u32;
                let input_length = input_triple.len() as u32;
                if acc_length == 0 {
                    let mut acc_msg = Msg::new();
                    {
                        let ip = acc_msg.build_schema::<list_triple::Builder>();
                        let mut acc_triple = ip.init_triples(input_length);
                        for i in 0..input_triple.len() {
                            acc_triple.borrow().get(i).set_first(input_triple.get(i).get_first()?);
                            acc_triple.borrow().get(i).set_second(input_triple.get(i).get_second()?);
                            acc_triple.borrow().get(i).set_third(input_triple.get(i).get_third()?);
                        }
                    }
                    self.output.accumulator.send(acc_msg)?;
                }else {
                    let mut medium_sized_bean_counter = HashMap::new();
                    for i in 0..input_triple.len() {
                        let first = input_triple.get(i).get_first()?;
                        let second = input_triple.get(i).get_second()?;
                        let third = input_triple.get(i).get_third()?;
                        if second.is_empty() || second == "0" {
                            continue;
                        } else {
                            let bean = medium_sized_bean_counter.entry(second).or_insert(0);
                            *bean += i32::from_str(third).unwrap();
                        }
                    }
                    for i in 0..acc_triple.len() {
                        let first = acc_triple.get(i).get_first()?;
                        let second = acc_triple.get(i).get_second()?;
                        let third = acc_triple.get(i).get_third()?;
                        if second.is_empty() || second == "0" {
                            continue;
                        } else {
                            let bean = medium_sized_bean_counter.entry(second).or_insert(0);
                            *bean += i32::from_str(third).unwrap();
                        }
                    }
                    let mut new_acc_msg = Msg::new();
                    {
                        let ip = new_acc_msg.build_schema::<list_triple::Builder>();
                        let mut new_acc_triple = ip.init_triples(medium_sized_bean_counter.len() as u32);
                        let first = acc_triple.get(0).get_first()?;
                        let mut i :u32 = 0;
                        for (key,val) in medium_sized_bean_counter.iter() {
                            new_acc_triple.borrow().get(i).set_first(first);
                            new_acc_triple.borrow().get(i).set_second(key);
                            new_acc_triple.borrow().get(i).set_third(format!("{}",val).as_str());
                            i += 1;
                        }
                    }
                    self.output.accumulator.send(new_acc_msg)?;
                }
            }
            let mut next_msg = Msg::new();
            {
                let mut ip = next_msg.build_schema::<value_string::Builder>();
                ip.set_value("next");
            }
            self.output.next.send(next_msg)?;
        }
        Ok(End)
    }
}
