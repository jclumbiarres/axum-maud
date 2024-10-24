use maud::{html, PreEscaped};

pub async fn root() -> PreEscaped<String> {
    html! {
        h1 { "Hola internet!" }
        p.intro {
            a href="https://github.com/lambda-fairy/maud" { "Maud" }
            " template language."
        }
    }
}
