use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    /// MAC address to send packet to. Should be in format AA:BB:CC:DD:EE:FF, AA-BB-CC-DD-EE-FF or
    /// AA/BB/CC/DD/EE/FF.
    mac_address: String,
}

fn main() -> wakey::Result<()> {
    let mac_adress = CmdArgs::parse().mac_address;
    let sep = mac_adress
        .chars()
        .find(|ch| *ch == ':' || *ch == '-' || *ch == '/')
        .expect("Invalid MAC address format. Please use one of the separators: [:, -, /]");
    let wol = wakey::WolPacket::from_string(&mac_adress, sep)?;
    if wol.send_magic().is_ok() {
        println!("Sent the magic packet.");
    } else {
        println!("Failed to send the magic packet.");
    }

    Ok(())
}
