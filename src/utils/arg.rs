use super::error::DbgError;



pub struct Arg {
    id: String,
    long: String,
    short: char,
    help: String,
    num_args: u64,
    takes_value: bool,
    required: bool,
    values: Vec<String>,
}


impl Arg {
    pub fn new(id: &str) -> Self {
        return Arg {
            id: id.to_string(),
            long: String::new(),
            short: '\0',
            help: String::new(),
            num_args: 0,
            takes_value: false,
            required: false,
            values: Vec::new(),
        };
    }


    pub fn long(mut self, long: &str) -> Self {
        self.long = long.to_string();
        return self;
    }


    pub fn short(mut self, short: char) -> Self {
        self.short = short;
        return self;
    }


    pub fn help(mut self, help: &str) -> Self {
        self.help = help.to_string();
        return self;
    }


    pub fn num_args(mut self, num_args: u64) -> Self {
        self.num_args = num_args;
        return self;
    }


    pub fn with_value(mut self, takes_value: bool) -> Self {
        self.takes_value = takes_value;
        return self;
    }


    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        return self;
    }


    pub fn push_value(&mut self, value: String) {
        self.values.push(value);
    }


    pub fn get_id(&self) -> &str {
        return &self.id;
    }


    pub fn get_long(&self) -> &str {
        return &self.long;
    }


    pub fn get_short(&self) -> char {
        return self.short;
    }


    pub fn get_help(&self) -> &str {
        return &self.help;
    }


    pub fn get_num_args(&self) -> u64 {
        return self.num_args;
    }


    pub fn takes_value(&self) -> bool {
        return self.takes_value;
    }


    pub fn is_required(&self) -> bool {
        return self.required;
    }


    pub fn get_values(&self) -> &Vec<String> {
        return &self.values;
    }
}



pub struct ArgParser {
    name: String,
    args: Vec<Arg>,
}


impl ArgParser {
    pub fn new(name: &str) -> Self {
        return ArgParser {
            name: name.to_string(),
            args: Vec::new(),
        };
    }
    

    pub fn add_arg(&mut self, arg: Arg) -> &mut Self {
        self.args.push(arg);
        return self;
    }


    pub fn get_name(&self) -> &str {
        return &self.name;
    }


    pub fn get_args(&self) -> &Vec<Arg> {
        return &self.args;
    }


    pub fn get_arg(&self, id: &str) -> Option<&Arg> {
        for arg in &self.args {
            if arg.get_id() == id {
                return Some(arg);
            }
        }
        return None;
    }


    pub fn parse_args(&mut self, input_args: Vec<String>) -> Result<(), DbgError> {
        let mut iter = input_args.iter().peekable();
        
        while let Some(arg) = iter.next() {
            let mut arg_str: &str = arg.as_str();
            
            if arg_str.starts_with("--") {
                arg_str = &arg_str[2..];
                match self.args.iter_mut().find(|a| a.get_long() == arg_str) {
                    Some(arg) => {
                        if arg.takes_value() {
                            for _ in 0..arg.num_args {
                                if let Some(value) = iter.next() {
                                    arg.push_value(value.clone());
                                } else {
                                    return Err(DbgError::new(&format!("Argument '{}' requires {} values", arg_str, arg.num_args)));
                                }
                            }
                        }
                    },
                    None => return Err(DbgError::new(&format!("Unknown argument: --{}", arg_str))),
                }
            } else if arg_str.starts_with("-") {
                arg_str = &arg_str[1..];
                if arg_str.chars().count() == 1 {
                } else {
                    return Err(DbgError::new(&format!("Invalid argument format: {}", arg_str)));
                }
                match self.args.iter_mut().find(|a| a.get_short() == arg_str.chars().next().unwrap()) {
                    Some(arg) => {
                        if arg.takes_value() {
                            for _ in 0..arg.get_num_args() {
                                if let Some(value) = iter.next() {
                                    arg.push_value(value.clone());
                                } else {
                                    return Err(DbgError::new(&format!("Argument '{}' requires {} values", arg_str, arg.num_args)));
                                }
                            }
                        }
                    },
                    None => return Err(DbgError::new(&format!("Unknown argument: -{}", arg_str))),
                }
            } else {
                return Err(DbgError::new(&format!("Invalid argument format: {}", arg_str)));
            }
        }
        
        Ok(())
    }
}