use  std::panic;
use color_eyre::{config::HookBuilder,eyre};
use crate::tui;

//replace standar color_eyre panic and error hooks wih hooks that restore the terminal before printing the panic or error
//mengganti print print an eror yang default

pub fn install_hooks() -> color_eyre::Result<()>{
    let (panic_hook,eyre_hook) = HookBuilder::default().into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ =tui::restore(); //restoring while also ignore any other errror because we are indeed error
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move|error:&(dyn std::error::Error + 'static)|{
            let _ = tui::restore();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}