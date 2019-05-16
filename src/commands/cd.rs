use crate::errors::ShellError;
use crate::prelude::*;
use derive_new::new;

#[derive(new)]
pub struct Cd;

impl crate::Command for Cd {
    fn run(&self, args: CommandArgs<'caller>) -> Result<VecDeque<ReturnValue>, ShellError> {
        let target = match args.args.first() {
            // TODO: This needs better infra
            None => return Err(ShellError::string(format!("cd must take one arg"))),
            Some(v) => v.as_string()?.clone(),
        };

        let cwd = args.env.cwd().to_path_buf();

        let mut stream = VecDeque::new();
        let path = dunce::canonicalize(cwd.join(&target).as_path())?;
        stream.push_back(ReturnValue::change_cwd(path));
        Ok(stream)
    }
}
