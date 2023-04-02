use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component]
pub fn Background() -> Html {
    html! {
        <div class={css!(
            r#"
                overflow: hidden;
                height: 100%;
                width: 100%;
                background: radial-gradient(circle, rgba(0,0,0,0.5) 50%, rgba(0,0,0,0) 90%);
                position: absolute;
                top: 0;
                left: 0;
                z-index: 1;
            "#
        )}>
            <div class={css!(
                r#"
                    height: 100%;
                    width: 100%;
                    background: radial-gradient(circle, rgba(0,0,0,0.5) 50%, rgba(0,0,0,0) 90%);
                    position: absolute;
                    top: 0;
                    left: 0;
                    z-index: 1;
                "#)}
            ></div>
            <div class={css!(
                r#"
                    background: url("../img/low_poly_experimental_snowy_cabin_with_aurora_bo.jpeg") repeat-x;
                    background-size: 50% 100%;
                    height: 100%;
                    width: 6144px; /* The image width times 2 */
                    animation: slide 720s linear infinite;

                    @keyframes slide{
                        0% {
                            transform: translate3d(0, 0, 0);
                        }
                        100% {
                            transform: translate3d(-3072px, 0, 0);
                        }
                    }
                "#
            )}></div>
        </div>
    }
}
