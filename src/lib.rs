#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod base64;
mod base_converter;
mod char_counter;
mod digest;
mod regex;
mod sudden_death;
mod util;

static ROOT: Option<&str> = option_env!("ROOT");

fn root() -> String {
    ROOT.unwrap_or("static").to_string()
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}

#[derive(Switch, Clone, PartialEq, Debug)]
enum AppRoute {
    #[to = "/{}/#/base64"]
    Base64(String),
    #[to = "/{}/#/digest"]
    Digest(String),
    #[to = "/{}/#/base-conv"]
    BaseConverter(String),
    #[to = "/{}/#/wc"]
    CharCounter(String),

    #[to = "/{}/#/regex"]
    Regex(String),
    #[to = "/{}/#/sudden-death"]
    SuddenDeath(String),

    #[to = "/{}/"]
    Index(String),
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header = html! {
            <nav class="navbar has-shadow is-spaced">
                <div class="navbar-brand">
                    <RouterAnchor<AppRoute> classes="navbar-item" route=AppRoute::Index(root())>
                        <h1 class="title is-3">{"SUGOI Tools"}</h1>
                    </RouterAnchor<AppRoute>>
                </div>

                <div class="navbar-end">
                <a class="bd-navbar-icon navbar-item" href="https://github.com/tanakh/tools" target="_blank">
                <span class="icon" style="color: #333;">
                    <i class="fab fa-lg fa-github-alt"></i>
                </span>
                </a>

                <a class="bd-navbar-icon navbar-item" href="https://twitter.com/tanakh" target="_blank">
                <span class="icon" style="color: #55acee;">
                    <i class="fab fa-lg fa-twitter"></i>
                </span>
                </a>
                </div>
            </nav>
        };

        let menu = html! {
            <aside class="menu">
                <p class="menu-label">
                    {"Tools"}
                </p>
                <ul class="menu-list">
                    <li><RouterLink text="Base64" route=AppRoute::Base64(root())/></li>
                    <li><RouterLink text="Message digest (MD5, SHA-1, SHA-2)" route=AppRoute::Digest(root())/></li>
                    <li><RouterLink text="Base converter" route=AppRoute::BaseConverter(root())/></li>
                    <li><RouterLink text="Character counter" route=AppRoute::CharCounter(root())/></li>
                </ul>

                <p class="menu-label">
                    {"Generators"}
                </p>
                <ul class="menu-list">
                    <li><RouterLink text="Regex Generator" route=AppRoute::Regex(root())/></li>
                    <li><RouterLink text="突然の死ジェネレーター" route=AppRoute::SuddenDeath(root())/></li>
                </ul>

                <p class="menu-label">
                    {"Underconstructions"}
                </p>
                <ul class="menu-list">
                    <li><a>{"ASCII converter"}</a></li>
                    <li><a>{"Prime factorization"}</a></li>
                    <li><a>{"URL encode"}</a></li>
                </ul>
            </aside>
        };

        let render = move |s| match s {
            AppRoute::Index(_) => html! {<IndexModel/>},
            AppRoute::Base64(_) => html! {<crate::base64::Model/>},
            AppRoute::Digest(_) => html! {<crate::digest::Model/>},
            AppRoute::BaseConverter(_) => html! {<crate::base_converter::Model/>},
            AppRoute::Regex(_) => html! {<crate::regex::Model/>},
            AppRoute::SuddenDeath(_) => html! {<crate::sudden_death::Model/>},
            AppRoute::CharCounter(_) => html! {<crate::char_counter::Model/>},
        };

        html! {
            <div style="display:flex;min-height:100vh;flex-direction:column;">

            { header }

            <section class="section" style="flex:1;">
                <div class="columns">
                    <div class="column is-one-quarter">
                        { menu }
                    </div>

                    <div class="column">
                        <Router<AppRoute, ()> render = Router::render(render) />
                    </div>
                </div>
            </section>

            <footer class="footer">
                <div class="content has-text-centered">
                <p>
                    {"Copyright (c) 2020, Hideyuki Tanaka"}
                </p>
                </div>
            </footer>

            </div>
        }
    }
}

struct RouterLink {
    props: RouteLinkProp,
}

#[derive(Properties, Clone)]
struct RouteLinkProp {
    text: String,
    route: AppRoute,
}

impl Component for RouterLink {
    type Message = ();
    type Properties = RouteLinkProp;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let route = self.props.route.clone();
        let text = self.props.text.clone();
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |s: AppRoute| {
                    html!{
                        <RouterAnchor<AppRoute> route=route.clone()
                            classes=if s == route {"is-active"} else {""}>
                            {text.clone()}
                        </RouterAnchor<AppRoute>>
                    }
                })
            />
        }
    }
}

struct IndexModel {}

impl Component for IndexModel {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h2 class="title is-2">
                {"👈(´･_･`👈) Select from here"}
            </h2>
        }
    }
}
