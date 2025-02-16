use address_checksum::address_checksum;
use clap::{Parser, Subcommand};
use secp256r1::{verify_signature_secp256r1, Secp256r1Args};

#[derive(Parser)]
#[command(name="odin",author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Odin {
    #[command(subcommand)]
    pub cmd: OdinSubcommand,
}

#[derive(Subcommand, Clone)]
pub enum OdinSubcommand {
    /// Computes the checksum of an Ethereum address
    #[command(visible_alias = "ac")]
    AddressChecksum { address: String },

    /// Verifies a secp256r1 signature
    #[command(visible_alias = "r1")]
    Secp256r1(Secp256r1Args),
}

fn main() {
    let args: Odin = Odin::parse();

    match args.cmd {
        OdinSubcommand::AddressChecksum { address } => {
            address_checksum(address);
        }
        OdinSubcommand::Secp256r1(args) => {
            let result = verify_signature_secp256r1(
                &args.pub_x,
                &args.pub_y,
                &args.message,
                &args.r,
                &args.s,
            );
            println!(
                "Signature verification result: {}",
                if result { "valid" } else { "invalid" }
            );
        }
    }
}
