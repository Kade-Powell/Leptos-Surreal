use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
use components::home::*;
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <body>
        // content for this welcome page
        <Router>
            <main class="container">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
        <footer class="container-fluid"> 
            <blockquote>
                "Maecenas vehicula metus tellus, vitae congue turpis hendrerit non. 
                Nam at dui sit amet ipsum cursus ornare."
                <footer>
                <cite>"- Phasellus eget lacinia"</cite>
                </footer>
            </blockquote>
        </footer>
        </body>
    }
}
