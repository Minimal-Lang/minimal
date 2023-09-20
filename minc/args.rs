use std::env::Args;

pub struct ArgsIter {
    args: Args,
}

pub enum Arg {
    Help,
    Emit(EmitList),

    InvalidEmit(String),
    Invalid(String),
}

pub struct EmitList {
    tokens: bool,

    ast: bool,
    bin_ast: bool,

    mir: bool,
    // binary MIR
    bytecode: bool,
}

impl ArgsIter {
    pub fn parse_emit_list(&mut self) -> Result<EmitList, String> {
        let mut ret = EmitList {
            tokens: false,
            ast: false,
            bin_ast: false,
            mir: false,
            bytecode: false,
        };

        if let Some(v) = self.args.next() {
            if v.as_str() == "*" {
                return Ok(EmitList {
                    tokens: true,
                    ast: true,
                    bin_ast: true,
                    mir: true,
                    bytecode: true,
                });
            }
            for (s, string) in v.split(',').map(|v| (v, v.to_string())) {
                match s {
                    "tokens" => ret.tokens = true,

                    "ast" => ret.ast = true,
                    "bin_ast" => ret.bin_ast = true,

                    "mir" => ret.mir = true,
                    "bytecode" => ret.bytecode = true,
                    _ => return Err(string),
                }
            }
        }

        Ok(ret)
    }
}

impl Iterator for ArgsIter {
    type Item = Arg;

    fn next(&mut self) -> Option<Self::Item> {
        let arg = self.args.next()?;
        Some(match arg.as_str() {
            "--help" => Arg::Help,
            "--emit" => match self.parse_emit_list() {
                Ok(v) => Arg::Emit(v),
                Err(e) => Arg::InvalidEmit(e),
            },
            _ => Arg::Invalid(arg),
        })
    }
}
