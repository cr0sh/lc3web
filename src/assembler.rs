use crate::util::{CrossComponentBridge, Props};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

pub struct AssemblerConsole {
    source: String,
    output: String,
    bridge: Rc<RefCell<CrossComponentBridge<Vec<u8>>>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CheckSource,
    AssembleAndLoad,
    SourceInput(String),
}

impl Component for AssemblerConsole {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            source: String::new(),
            output: String::new(),
            bridge: props.bridge,
            link,
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

    fn view(&self) -> Html {
        let cb0 = self.link.callback(|e: InputData| Msg::SourceInput(e.value));
        let cb1 = self.link.callback(|_| Msg::CheckSource);
        let cb2 = self.link.callback(|_| Msg::AssembleAndLoad);
        html! {
            <div>
                { "Source code" }
                <br />
                <textarea id="asm-source" rows=15 cols=25 value=&self.source oninput=cb0 />
                <textarea readonly=true id="asm-output" rows=15 cols=25>
                    { &self.output }
                </textarea>
                <button onclick = cb1>{ "Check" }</button>
                <button onclick = cb2>{ "Assemble and load" }</button>
            </div>
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> ShouldRender {
        false
    }
}
