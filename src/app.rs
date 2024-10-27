use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use crate::i18n::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let error_page = || {
	let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! {
          <ErrorTemplate outside_errors/>
        }.into_view()
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/i18n.css"/>

	<I18nContextProvider>
            <Router>
                <main>
                    <Routes fallback=error_page>
                        <I18nRoute view=Outlet>
                            <Route path=path!("") view=AepaPage/>
                            <Route path=path!("/aepa") view=AepaPage/>
                        </I18nRoute>
                    </Routes>
                </main>
            </Router>
        </I18nContextProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn AepaPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
