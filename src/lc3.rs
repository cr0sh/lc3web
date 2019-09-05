use crate::util::Props;
use lc3::{IOStreamHandler, VM};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::io::{Cursor, Read, Result as IOResult};
use yew::prelude::*;
use yew::worker::*;

pub struct LC3Console {
    display: String,
    input: String,
    context: Box<dyn Bridge<LC3Agent>>,
}

pub enum Msg {
    AgentResponse(String),
    LoadSample,
    Run,
    LoadFromAssembler(Vec<u8>),
    Clear,
    Input(String),
}

impl Component for LC3Console {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|x: <LC3Agent as Agent>::Output| Msg::AgentResponse(x.0));
        props
            .bridge
            .borrow_mut()
            .register_callback(link.send_back(|v| {
                stdweb::console!(log, "Received program!");
                Msg::LoadFromAssembler(v)
            }));
        stdweb::console!(log, "Registered CCB callback");
        LC3Console {
            display: String::new(),
            input: String::new(),
            context: LC3Agent::bridge(callback),
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
}

impl Renderable<Self> for LC3Console {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                { "Output" }
                <br />
                <textarea id="lc3-output" rows=15 cols=25 value=&self.display />
                <button onclick = |_| Msg::LoadSample>{ "Load sample program" }</button>
                <br />
                { "Input" }
                <br />
                <textarea id="lc3-input" rows=15 cols=25 value=&self.input oninput=|e| Msg::Input(e.value) />
                <button onclick = |_| Msg::Run>{ "Run" }</button>
                <button onclick = |_| Msg::Clear>{ "Clear output screen" }</button>
            </div>
        }
    }
}

#[derive(Serialize, Deserialize)]
enum AgentRequest {
    Load(Vec<u8>),
    Run(String),
    Reset,
}
impl Transferable for AgentRequest {}

#[derive(Serialize, Deserialize)]
struct AgentResponse(String);
impl Transferable for AgentResponse {}

struct LC3Agent {
    lc3_vm: Box<VM<IOStreamHandler<StringCell, Vec<u8>>>>,
    link: AgentLink<Self>,
}

impl Agent for LC3Agent {
    type Reach = Context;
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

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            AgentRequest::Load(program) => {
                self.lc3_vm.load_u8(&program);
                self.link
                    .response(who, AgentResponse(String::from("Loaded program...\n")))
            }
            AgentRequest::Run(input) => {
                (self.lc3_vm.context.0).0.set(Cursor::new(input));
                self.lc3_vm.run().unwrap();
                self.link.response(
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
                self.link.response(who, AgentResponse(String::from("VM reset complete\n")))
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
