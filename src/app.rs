use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
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
            <Router fallback=error_page>
                <main>
                    <Routes>
                        <I18nRoute view=Outlet>
                            <Route path="" view=AepaPage/>
                            <Route path="/aepa" view=AepaPage/>
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
    let i18n = use_i18n();

    let title = t!(i18n, aepa.meta_title);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
	<Title text=title/>

        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn BPage() -> impl IntoView {
    let i18n = use_i18n();

    let q = "my-q-search-param";

    let title = move || t_string!(i18n, auraa.meta_title, q = q);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
	<Title text=title/>

        <h1>"B Page!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn CPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = t!(i18n, putatoro.meta_title);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
	<Title text=title/>

        <h1>"C Page!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn DPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = t!(i18n, e404.meta_title);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
	<Title text=title/>

        <h1>"D Page!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
