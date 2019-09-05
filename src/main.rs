use lc3web::assembler::AssemblerConsole;
use lc3web::lc3::LC3Console;
use lc3web::util::CrossComponentBridge;
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

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
}

impl Renderable<RootModel> for RootModel {
    fn view(&self) -> Html<Self> {
        html! {
        <div id="root-element">
            <div class="outer-container">
                <label>{"LC-3 Console"}</label>
                <div class="container">
                    <LC3Console bridge=Rc::clone(&self.0) />
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
}

fn main() {
    yew::start_app::<RootModel>()
}
