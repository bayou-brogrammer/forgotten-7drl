use super::*;

#[derive(Args)]
pub struct AnsiTerminal {
    #[clap(short, long, action)]
    encode: ColEncodeChoice,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColEncodeChoice {
    Rgb,
    AnsiColor,
    Greyscale,
    TrueColor,
}

impl AnsiTerminal {
    pub fn run(&self, app: gridbugs::chargrid::control_flow::App) {
        use gridbugs::chargrid_ansi_terminal::*;

        let ctx = Context::new().unwrap();
        match self.encode {
            ColEncodeChoice::Rgb => ctx.run(app, col_encode::FromTermInfoRgb),
            ColEncodeChoice::AnsiColor => ctx.run(app, col_encode::FromTermInfoRgb),
            ColEncodeChoice::TrueColor => ctx.run(app, col_encode::XtermTrueColour),
            ColEncodeChoice::Greyscale => ctx.run(app, col_encode::FromTermInfoGreyscale),
        }
    }
}
