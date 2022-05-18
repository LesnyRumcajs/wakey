use clap::Parser;

#[derive(Parser)]
#[clap(about = "WakeOnLan a device. https://github.com/LesnyRumcajs/wakey.git", long_about = None)]
struct CmdArgs {
    /// mac address to send packet to
    #[clap(short, long)]
    mac: Option<String>,
}
fn main() {
    let args = CmdArgs::parse();
    if let Some(m) = args.mac {
        let mut sep: char = '/';
        [":", "-"].iter().for_each(|s| {
            if m.contains(s) {
                sep = s.chars().next().unwrap();
            }
        });
        let wol = wakey::WolPacket::from_string(&m, sep);
        if wol.send_magic().is_ok() {
            println!("sent the magic packet.");
        } else {
            println!("failed to send the magic packet.");
        }
    } else {
        println!("give mac address to wake up");
    }
}
