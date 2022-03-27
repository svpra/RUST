#![windows_subsystem = "windows"]
 
mod bindings {
    windows::include_bindings!();
}
 
use bindings::*;
use windows::*;
 
use bindings::{
    Windows::ApplicationModel::Activation::*, 
    Windows::UI::Xaml::Controls::TextBlock,
    Windows::UI::Xaml::*,
};
 
#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched
)]
struct MyApp();
 
#[allow(non_snake_case)]
impl MyApp {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
         
        let textBlock: TextBlock = TextBlock::new()?;
        textBlock.SetText("Hello Rust from Metanit.com")?;
        textBlock.SetFontSize(22.0)?;
         
        window.SetContent(textBlock)?;
        window.Activate()
    }
}
 
 
fn main() -> Result<()> {
    initialize_mta()?;
    Application::Start(ApplicationInitializationCallback::new(|_| {
        MyApp().new()?;
        Ok(())
    }))
}
