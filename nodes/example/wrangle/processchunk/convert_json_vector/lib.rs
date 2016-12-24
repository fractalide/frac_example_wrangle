#[macro_use]
extern crate rustfbp;
extern crate capnp;
extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcDecodable)]
pub struct Item {
    thetype: String,
    amount: i64,
}
#[derive(RustcDecodable)]
struct Purchases {
    purchases: Vec<Item>,
}

agent! {
    input(input: value_string),
    output(output: list_ntuple_tuple_tt),
    fn run(&mut self) -> Result<Signal> {
        let mut ip = self.input.input.recv()?;
        let value: value_string::Reader = ip.read_schema()?;
        let value = value.get_value()?;
        if value != "end" {
            if value.contains("type") {
                let purchases: Purchases = json::decode(value.replace("type", "thetype").as_str()).unwrap();
                let purchases = Purchases {purchases:  purchases.purchases};
                let mut new_msg = Msg::new();
                {
                    let ip = new_msg.build_schema::<list_ntuple_tuple_tt::Builder>();
                    let mut tuples = ip.init_list(purchases.purchases.len() as u32);
                    let mut i: u32 = 0;
                    for tuple in &purchases.purchases {
                        tuples.borrow().get(i).get_first()?.set_text(tuple.thetype.as_str());
                        tuples.borrow().get(i).get_second()?.set_text(format!("{}",tuple.amount).as_str());
                        i += 1;
                    }
                }
                self.output.output.send(new_msg)?;
            }else {
                let mut empty_msg = Msg::new();
                {
                    let ip = empty_msg.build_schema::<list_ntuple_tuple_tt::Builder>();
                    let mut tuples = ip.init_list(1);
                    tuples.borrow().get(0).get_first()?.set_text("zero");
                    tuples.borrow().get(0).get_second()?.set_text("0");
                }
                self.output.output.send(empty_msg)?;
            }
        } else {
            let mut end_ip = Msg::new();
            {
                let ip = end_ip.build_schema::<list_ntuple_tuple_tt::Builder>();
                let mut tuples = ip.init_list(1);
                tuples.borrow().get(0).get_first()?.set_text("end");
            }
            self.output.output.send(end_ip)?;
        }
        Ok(End)
    }
}
