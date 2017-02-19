#[macro_use]
extern crate rustfbp;
extern crate capnp;

agent! {
    input(input: fs_list_path),
    outarr(output: fs_list_path),
    fn run(&mut self) -> Result<Signal> {
        let mut msg = try!(self.input.input.recv());
        let list: fs_list_path::Reader = try!(msg.read_schema());
        let list = list.get_list()?;

        let mut v = Vec::with_capacity(list.len() as usize);
        for i in 0..list.len()
        {
            v.push(list.get(i).get_path()?);
        }
        let out_array_count = self.outarr.output.keys().count();
        let inc_by = list.len() as usize / out_array_count ;
        let mut i: u32 = 0;
        for chunk in v.chunks(inc_by)
        {
            let mut new_msg = Msg::new();
            {
                let msg = new_msg.build_schema::<fs_list_path::Builder>();
                let mut files = msg.init_list(chunk.len() as u32);
                let mut i: u32 = 0;
                for path in chunk {
                    files.borrow().get(i).set_path(path);
                    i += 1;
                }
            }
            if let Some(sender) = self.outarr.output.get(&format!("{}", i)){
                try!(sender.send(new_msg));
            }
            i += 1;
        }
        Ok(End)
    }
}
