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
        let sep = m.chars().find(|ch| *ch == ':' || *ch == '-').unwrap_or('/');
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
