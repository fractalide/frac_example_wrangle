#[macro_use]
extern crate rustfbp;
extern crate capnp;

agent! {
    input(input: fs_list_path, next: prim_text),
    output(output: fs_path),
    fn run(&mut self) -> Result<Signal> {
        let mut msg = self.input.input.recv()?;
        let list: fs_list_path::Reader = msg.read_schema()?;
        let list = list.get_list()?;
        for i in 0..list.len()
        {
            println!("current file: {:?} of {:?}", i, list.len());
            let mut new_msg = Msg::new();
            {
                let mut msg = new_msg.build_schema::<fs_path::Builder>();
                msg.set_path(list.get(i).get_path()?);
            }
            self.output.output.send(new_msg)?;
            let mut msg = self.input.next.recv()?;
        }

        let mut end_msg = Msg::new();
        {
            let mut msg = end_msg.build_schema::<fs_path::Builder>();
            msg.set_path("end");
        }
        self.output.output.send(end_msg)?;
        Ok(End)
    }
}
