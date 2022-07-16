use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    /// the port number to be used to start the udp
    /// server which will listen any commands to be
    /// sent from polybar.
    #[clap(short = 'p', value_parser, default_value_t=33333)]
    pub port: u32,
   
    /// A dummy argument for the polybar to operate on the
    /// actions
    #[clap(short = 'a', default_value="")]
    action: String,
}


