#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

agent! {
    input(input: path),
    output(output: value_string, error: file_error),
    fn run(&mut self) -> Result<Signal> {
        let mut ip = self.input.input.recv()?;
        let path: path::Reader = ip.read_schema()?;

        let path = path.get_path()?;

        if path == "end" {
            let mut new_msg = Msg::new();
            {
                let mut ip = new_msg.build_schema::<value_string::Builder>();
                ip.set_value("end");
            }
            try!(self.output.output.send(new_msg));
        }
        else {
            let file = match File::open(path) {
                Ok(file) => { file },
                Err(_) => {
                    let mut new_msg = Msg::new();
                    {
                        let mut ip = new_msg.build_schema::<file_error::Builder>();
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
                    let mut ip = new_msg.build_schema::<value_string::Builder>();
                    ip.set_value(&l);
                }
                try!(self.output.output.send(new_msg));
            }
        }
        Ok(End)
    }
}
