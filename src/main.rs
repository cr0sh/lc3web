use lc3web::assembler::AssemblerConsole;
use lc3web::lc3::Lc3Console;
use lc3web::util::CrossComponentBridge;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

struct RootModel(Rc<RefCell<CrossComponentBridge<Vec<u8>>>>);

impl Component for RootModel {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self(Rc::new(RefCell::new(CrossComponentBridge::new())))
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div id="root-element">
            <div class="outer-container">
                <label>{"LC-3 Console"}</label>
                <div class="container">
                    <Lc3Console bridge=Rc::clone(&self.0) />
                </div>
            </div>
            <div class="outer-container">
                <label>{"Assembler Console"}</label>
                <div class="container">
                    <AssemblerConsole bridge=Rc::clone(&self.0) />
                </div>
            </div>
        </div>
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> ShouldRender {
        true
    }
}

fn main() {
    yew::start_app::<RootModel>()
}
