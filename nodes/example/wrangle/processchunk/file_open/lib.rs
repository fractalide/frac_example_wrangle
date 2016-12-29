#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

agent! {
    input(input: fs_path),
    output(output: prim_text, error: fs_file_error),
    fn run(&mut self) -> Result<Signal> {
        let mut ip = self.input.input.recv()?;
        let path: fs_path::Reader = ip.read_schema()?;

        let path = path.get_path()?;

        if path == "end" {
            let mut new_msg = Msg::new();
            {
                let mut ip = new_msg.build_schema::<prim_text::Builder>();
                ip.set_text("end");
            }
            try!(self.output.output.send(new_msg));
        }
        else {
            let file = match File::open(path) {
                Ok(file) => { file },
                Err(_) => {
                    let mut new_msg = Msg::new();
                    {
                        let mut ip = new_msg.build_schema::<fs_file_error::Builder>();
                        ip.set_not_found(&path);
                    }
                    let _ = self.output.error.send(new_msg);
                    return Ok(End);
                }
            };

            let file = BufReader::new(&file);
            for line in file.lines() {
                let l = try!(line);
                let mut new_msg = Msg::new();
                {
                    let mut ip = new_msg.build_schema::<prim_text::Builder>();
                    ip.set_text(&l);
                }
                try!(self.output.output.send(new_msg));
            }
        }
        Ok(End)
    }
}
