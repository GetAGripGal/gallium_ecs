//! Provides macros for easier component creation

/** Proc-macro attribute for components */
#[proc_macro_attribute]
pub fn component(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Get the name
    let p_input = input.to_string();
    let p_input = p_input.split(' ').collect::<Vec<&str>>();

    let name_index = p_input
        .iter()
        .enumerate()
        .find(|t| t.1 == &"struct")
        .unwrap()
        .0
        + 1;

    let name = p_input[name_index];

    let output = format!(
        r#"
        #[derive(Serialize, Deserialize)]
        {}
        #[typetag::serde]
        impl Component for {} {{
            fn as_any(&self) -> &dyn std::any::Any {{
                return self;
            }}

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {{
                return self;
            }}
        }}
    "#,
        input, name
    );
    return output.parse().unwrap();
}

/** Proc-macro attribute for systems */
#[proc_macro_attribute]
pub fn system(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return format!(
        r#"
        #[typetag::serde]
        {}
    "#,
        input
    )
    .parse()
    .unwrap();
}
