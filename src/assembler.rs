use crate::util::{CrossComponentBridge, Props};
use yew::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AssemblerConsole {
    source: String,
    output: String,
    bridge: Rc<RefCell<CrossComponentBridge<Vec<u8>>>>,
}

pub enum Msg {
    CheckSource,
    AssembleAndLoad,
    SourceInput(String),
}

impl Component for AssemblerConsole {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            source: String::new(),
            output: String::new(),
            bridge: props.bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CheckSource => {
                self.output.clear();
                self.output.push_str("Checking...");
                match lc3asm::assemble(&self.source) {
                    Ok(_) => {
                        self.output.push_str("Success\n");
                    }
                    Err(e) => {
                        self.output.push_str("Failed\n");
                        self.output.push_str(&e.to_string());
                    }
                }
                true
            }
            Msg::AssembleAndLoad => {
                self.output.clear();
                self.output.push_str("Assembling...");
                match lc3asm::assemble(&self.source) {
                    Ok((obj, _)) => {
                        self.output.push_str("Success\n");
                        self.output.push_str("Sending to LC-3 console...");
                        self.bridge.borrow().send(obj);
                        self.output.push_str("Done\n");
                    }
                    Err(e) => {
                        self.output.push_str("Failed\n");
                        self.output.push_str(&e.to_string());
                    }
                }
                true
            }
            Msg::SourceInput(value) => {
                self.source = value;
                true
            }
        }
    }
}

impl Renderable<Self> for AssemblerConsole {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                { "Source code" }
                <br />
                <textarea id="asm-source" rows=15 cols=25 value=&self.source oninput=|e| Msg::SourceInput(e.value) />
                <textarea readonly=true id="asm-output" rows=15 cols=25>
                    { &self.output }
                </textarea>
                <button onclick = |_| Msg::CheckSource>{ "Check" }</button>
                <button onclick = |_| Msg::AssembleAndLoad>{ "Assemble and load" }</button>
            </div>
        }
    }
}
