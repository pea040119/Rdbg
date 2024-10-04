use ansi_term::Colour;



pub fn error_header() -> ansi_term::ANSIGenericString<'static, str> {
    return Colour::Red.paint("[!] Error");
}


pub fn success_header() -> ansi_term::ANSIGenericString<'static, str> {
    return Colour::Green.paint("[+] Success");
}