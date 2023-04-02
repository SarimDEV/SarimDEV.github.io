use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

mod background;
use background::Background;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub icon_id: IconId,
    pub url: AttrValue,
}

#[styled_component]
fn Link(props: &LinkProps) -> Html {
    html! {
        <a href={props.url.clone()} target="_blank" rel="noopener noreferrer" class={css!("color: inherit; text-decoration: inherit;")}>
            <Icon class={css!("margin-right: 8px;")}icon_id={props.icon_id}/>
        </a>
    }
}

#[styled_component]
fn Links() -> Html {
    html! {
        <>
            <Link icon_id={IconId::BootstrapGithub} url={"https://github.com/sarimdev"}/>
            <Link icon_id={IconId::LucideMail} url={"mailto:m2sarim@gamil.com"}/>
            <Link icon_id={IconId::BootstrapLinkedin} url={"https://www.linkedin.com/in/sarim-dev/"}/>
        </>

    }
}

#[styled_component]
fn Content() -> Html {
    html! {
        <div class={css!(
            r#"
                height: 100%;
                width: 100%;
                display: flex;
                justify-content: center;
                align-items: center;
            "#
        )}>
            <div class={css!(
                r#"
                    width: 500px;
                "#
            )}>
                <h1>{"Muhammad Sarim"}</h1>
                <p>{"Hi 👋, I’m a software engineer that enjoys working on difficult problems and working on impactful products."}</p>
                <Links />
            </div>
        </div>

    }
}

#[styled_component]
fn App() -> Html {
    html! {
        <>
            <Global css={css!(
                r#"
                    html, body {
                        font-family: Verdana;
                        padding: 0;
                        margin: 0;
                        font-size: 1.1rem;
                        width: 100%;
                        min-height: 100vh;
                        height: 1vh;
                        background-color: #F1F1F1;
                        color: white;
                    }
                "#
            )} />
            <Background />
            <div class={css!(
                r#"
                    height: 100%;
                    width: 100%;
                    position: absolute;
                    top: 0;
                    left: 0;
                    z-index: 2;
                "#
            )}><Content /></div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
