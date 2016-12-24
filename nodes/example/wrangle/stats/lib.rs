#![feature(btree_range, collections_bound)]
#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::collections::BTreeSet;
use std::collections::Bound::{Included, Unbounded, Excluded};
use std::str::FromStr;

fn process_data(mut msg: rustfbp::ports::Msg) -> Result<(u32,u32,u32,f32)>
{
    let data_reader: list_ntuple_triple_ttt::Reader = msg.read_schema()?;
    let data = data_reader.get_list()?;
    let stats_length = data.len();
    let mut total :u32 = 0;
    let mut stats = BTreeSet::new();
    for i in 0..data.len() {
        let second = u32::from_str(data.get(i).get_second()?.get_text()?).unwrap();
        stats.insert(second);
        total += second;
    }
    let min =  stats.iter().next().unwrap();
    let max =  stats.iter().last().unwrap();
    let average :u32 = total / stats.len() as u32;
    let median :f32 = if stats.len() as f32 % 2 as f32 != 0 as f32{
        let mid :f32 = stats.len() as f32 / 2 as f32;
        let floor = mid.floor();
        let ceil = mid.ceil();
        let floor_val = stats.iter().nth(floor as usize).unwrap().clone();
        let ceil_val = stats.iter().nth(ceil as usize).unwrap().clone();
        floor_val as f32 + ceil_val as f32 / 2 as f32
    } else {
        stats.iter().nth(stats.len()/2).unwrap().clone() as f32
    };
    Ok((min.clone(),max.clone(),average.clone(),median.clone()))
}

agent! {
    input(raw: list_ntuple_triple_ttt, anonymous: list_ntuple_triple_ttt),
    output(raw: ntuple_quadruple_u32u32u32f32, anonymous: ntuple_quadruple_u32u32u32f32),
    fn run(&mut self) -> Result<Signal> {
        let (min, max, average, median): (u32, u32, u32, f32) = process_data(self.input.raw.recv()?)?;
        let mut raw_msg = Msg::new();
        {
            let mut quad = raw_msg.build_schema::<ntuple_quadruple_u32u32u32f32::Builder>();
            quad.borrow().get_first()?.set_u32(min);
            quad.borrow().get_second()?.set_u32(max);
            quad.borrow().get_third()?.set_u32(average);
            quad.borrow().get_fourth()?.set_f32(median);
        }
        self.output.raw.send(raw_msg)?;

        let (min, max, average, median): (u32, u32, u32, f32) = process_data(self.input.anonymous.recv()?)?;
        let mut anonymous_msg = Msg::new();
        {
            let mut quad = anonymous_msg.build_schema::<ntuple_quadruple_u32u32u32f32::Builder>();
            quad.borrow().get_first()?.set_u32(min);
            quad.borrow().get_second()?.set_u32(max);
            quad.borrow().get_third()?.set_u32(average);
            quad.borrow().get_fourth()?.set_f32(median);
        }
        self.output.anonymous.send(anonymous_msg)?;
        Ok(End)
    }
}
