use crate::util::Props;
use lc3::{IOStreamHandler, VM};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::io::{Cursor, Read, Result as IOResult};
use yew::prelude::*;
use yew::worker::*;

pub struct Lc3Console {
    display: String,
    input: String,
    context: Box<dyn Bridge<Lc3Agent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AgentResponse(String),
    LoadSample,
    Run,
    LoadFromAssembler(Vec<u8>),
    Clear,
    Input(String),
}

impl Component for Lc3Console {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|x: <Lc3Agent as Agent>::Output| Msg::AgentResponse(x.0));
        props
            .bridge
            .borrow_mut()
            .register_callback(link.callback(Msg::LoadFromAssembler));
        Lc3Console {
            display: String::new(),
            input: String::new(),
            context: Lc3Agent::bridge(callback),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadSample => {
                self.display.clear();
                self.context.send(AgentRequest::Reset);
                self.context.send(AgentRequest::Load(Vec::from(
                    include_bytes!("print.obj").as_ref(),
                )));
                true
            }
            Msg::Run => {
                self.display.clear();
                self.context.send(AgentRequest::Run(self.input.clone()));
                false
            }
            Msg::Clear => {
                self.display.clear();
                true
            }
            Msg::LoadFromAssembler(v) => {
                self.display.clear();
                self.display.push_str("Loading assembled file...\n");
                self.context.send(AgentRequest::Reset);
                self.context.send(AgentRequest::Load(v));
                true
            }
            Msg::AgentResponse(s) => {
                self.display.push_str(&s);
                true
            }
            Msg::Input(value) => {
                self.input = value;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let cb0 = self.link.callback(|_| Msg::LoadSample);
        let cb1 = self.link.callback(|e: InputData| Msg::Input(e.value));
        let cb2 = self.link.callback(|_| Msg::Run);
        let cb3 = self.link.callback(|_| Msg::Clear);
        html! {
            <div>
                { "Output" }
                <br />
                <textarea id="lc3-output" rows=15 cols=25 value=&self.display />
                <button onclick = cb0>{ "Load sample program" }</button>
                <br />
                { "Input" }
                <br />
                <textarea id="lc3-input" rows=15 cols=25 value=&self.input oninput = cb1 />
                <button onclick = cb2>{ "Run" }</button>
                <button onclick = cb3>{ "Clear output screen" }</button>
            </div>
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> ShouldRender {
        true
    }
}

#[derive(Serialize, Deserialize)]
enum AgentRequest {
    Load(Vec<u8>),
    Run(String),
    Reset,
}

#[derive(Serialize, Deserialize)]
struct AgentResponse(String);

struct Lc3Agent {
    lc3_vm: Box<VM<IOStreamHandler<StringCell, Vec<u8>>>>,
    link: AgentLink<Self>,
}

impl Agent for Lc3Agent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = AgentRequest;
    type Output = AgentResponse;

    fn create(link: AgentLink<Self>) -> Self {
        let vm = VM::new((
            StringCell(Cell::new(Cursor::new(String::new()))),
            Vec::new(),
        ));
        Self {
            lc3_vm: Box::new(vm),
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            AgentRequest::Load(program) => {
                self.lc3_vm.load_u8(&program);
                self.link
                    .respond(who, AgentResponse(String::from("Loaded program...\n")))
            }
            AgentRequest::Run(input) => {
                (self.lc3_vm.context.0).0.set(Cursor::new(input));
                self.lc3_vm.run().unwrap();
                self.link.respond(
                    who,
                    AgentResponse(String::from_utf8_lossy(&self.lc3_vm.context.1).into_owned()),
                )
            }
            AgentRequest::Reset => {
                let vm = VM::new((
                    StringCell(Cell::new(Cursor::new(String::new()))),
                    Vec::new(),
                ));
                self.lc3_vm = Box::new(vm);
                self.link
                    .respond(who, AgentResponse(String::from("VM reset complete\n")))
            }
        }
    }
}

struct StringCell(Cell<Cursor<String>>);

impl Read for StringCell {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.0.get_mut().read(buf)
    }
}
