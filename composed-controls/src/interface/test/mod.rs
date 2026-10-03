#[test]
fn public_composed_controls_workflow() {
    super::headless().expect("public UI workflow");
}

#[test]
fn public_nested_layers_workflow() {
    super::layers::headless().expect("nested public UI layers");
}

#[test]
fn public_animation_workflow() {
    super::animation::headless().expect("public UI transitions");
}
