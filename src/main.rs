use leptos::*;
use web_sys::SubmitEvent;

fn main() {
    // mount_to_body(|| view! { <AppComponetProp/> })
    mount_to_body(App)
}

// test App
#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>
    }
}

// 3.1 Basic Components

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

// 3.2 Dynamic Attributes

#[component]
fn AppDynamicAttribute() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
                set_x.update(|n| *n += 10);
                set_y.update(|n| *n += 10);
            }

            class:red=move || { count() % 2 == 0 }
        >

            "Click me!"
            {count}
        </button>
        <button on:click=move |_| {
            set_count.update(|n| *n = 0);
            set_x.update(|n| *n = 0);
            set_y.update(|n| *n = 0);
        }>

            "Reset"
        </button>
        <div
            style="positon: absolute"
            style:left=move || format!("{}px", x() + 100)
            style:top=move || format!("{}px", y() + 100)
            style:background-color=move || {
                format!("rgb({}, {}, 100)", x(), y())
            }
            style=("--columns", x)
        >
            "Move when coordinates change"
        </div>
        <progress max="50" value=move || count() * 2></progress>
    }
}

// 3.3 Components and Props

#[component]
fn ProgressBar1(progress: ReadSignal<i32>, // not a plain i32
) -> impl IntoView {
    view! { <progress max="50" value=progress></progress> }
}

#[component]
fn AppComponentProp1() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <div>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1)
            }>"Click me! Using Generic" {count}</button>
            <br/>
            <ProgressBar2 progress=count/>
            <br/>
            <ProgressBar2 progress=double_count/>
        </div>
    }
}

#[component]
fn ProgressBar2<F>(
    #[prop(default = 100)] 
    max: u16, 
    progress: F
) -> impl IntoView
where
    F: Fn() -> i32 + 'static, // can't be specified an `impl` yet
{
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn ProgressBar3(
    #[prop(default = 100)] 
    max: u16, 
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn AppComponentProp2() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <div>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1)
            }>"Click me! Using into" {count}</button>
            <br/>
            <ProgressBar3 progress=count/>
            <br/>
            <ProgressBar3 progress=Signal::derive(double_count)/>
        </div>
    }
}

/// Shows progress of a task
#[component]
fn ProgressBar4(
    /// Maximum value of the progress bar
    #[prop(default = 100)] 
    max: u16, 
    /// How much progress has been made
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! { <progress max=max value=progress></progress> }
}

// 3.4 Iteration
#[component]
fn AppIterStatic() -> impl IntoView {
    let items = vec!["a", "b", "c"];
    view! {
        <p>
            "Static Iteration"
            {items.clone()}
        </p>
        <ul>
            {items.into_iter()
                .map(|item| view! { <li>{item}</li> })
                .collect::<Vec<_>>()
            }
        </ul>
    }
}


// 3.5 Iterating more complex data
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn AppIterComplex() -> impl IntoView {
    // start with a set of 3 rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 15,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 20,
        },
    ]);
    view! {
        // when click, update each row
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data|{
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}

#[component]
pub fn AppIterComplexModifiedMemo() -> impl IntoView {
    // start with a set of 3 rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 15,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 30,
        },
    ]);
    view! {
        // when click, update each row
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data|{
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=move || data().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntrySignal {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn AppIterComplexModifiedSignal() -> impl IntoView {
    // start with a set of 3 rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntrySignal {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntrySignal {
            key: "bar".to_string(),
            value: create_rw_signal(15),
        },
        DatabaseEntrySignal {
            key: "baz".to_string(),
            value: create_rw_signal(30),
        },
    ]);
    view! {
        // when click, update each row
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data|{
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}

// 3.6 Forms and Inputs

// Controlled Input
#[component]
fn ControlledComponent() -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <input type="test"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // if functions the same way as `event.target.value`
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev))
            }
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is " {name}</p>
    }
}

// Uncontrolled Input
#[component]
fn UncontrolledComponent() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // We'll use a NodeRef to store a reference to the input element
    // this will filled when the element is created.
    let input_element: NodeRef<Input> = create_node_ref();

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this menas we can call `HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                // here, we use the `value` **attribute** to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=name
                //store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit"/>
        </form>
        <p>"Name is " {name}</p>
    }
}