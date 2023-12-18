use leptos::*;

fn main() {
    mount_to_body(|| view! { <AppComponetProp/> })
}

#[component]
fn AppBasic() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1)
        }>

            // nightly only
            "Click me!" {count}
        // stable
        // "Click me!" {move || count.get() }
        </button>
    }
}

#[component]
fn AppDynamicAttribute() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);

    view! {
        <button 
            on:click = move |_| {
                set_count.update(|n| *n += 1);
                set_x.update(|n| *n += 10);
                set_y.update(|n| *n += 10);
            }
            class:red = move || { count() % 2 == 0 }
        >

            "Click me!" {count}
        </button>
        <button 
            on:click = move |_| {
                set_count.update(|n| *n = 0);
                set_x.update(|n| *n = 0);
                set_y.update(|n| *n = 0);
            }
        >
                "Reset"
        </button>
        <div
            style="positon: absolute"
            style:left = move || format!("{}px", x() + 100)
            style:top = move || format!("{}px", y() + 100)
            style:background-color = move || format!("rgb({}, {}, 100)", x(), y())
            style=("--columns", x)
        >
            "Move when coordinates change"
        </div>
        <progress max="50" value = move || count() * 2 />
    }
}

// 3.3 Components and Props
#[component]
fn ProgressBar(
    progress: ReadSignal<i32>  // not a plain i32
) -> impl IntoView {
    view! {
        <progress max="50" value = progress />
    }
}

#[component]
fn AppComponetProp() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <div>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1)
            }>
                "Click me!" {count}
            </button>
            <br />
            <ProgressBar2 progress=count />
            <br />
            <ProgressBar2 progress=double_count />
        </div>
    }
}

#[component]
fn ProgressBar2<F> (
    #[prop(default = 100)]
    max: u16,
    progress: F
) -> impl IntoView 
where
    F: Fn() -> i32 + 'static // can't be specified an `impl` yet
{
    view! {
        <progress max=max value = progress />
    }
}