use slaq::blocks::elements::*;
use slaq::blocks::*;

fn main() -> Result<(), BuildError> {
    // Actions block showcasing common interactive elements
    let select = StaticSelectElement::new("select_1")
        .placeholder(PlainText::new("Pick one"))
        .options(vec![
            SelectOption::new(PlainText::new("A"), "a"),
            SelectOption::new(PlainText::new("B"), "b"),
        ]);

    let external = ExternalSelectElement::new("ext_1").min_query_length(2);
    let button = ButtonElement::new(PlainText::new("Confirm"), "btn_confirm");
    let overflow = OverflowElement::new(
        "more",
        vec![
            SelectOption::new(PlainText::new("Edit"), "edit"),
            SelectOption::new(PlainText::new("Delete"), "delete"),
        ],
    );

    let actions = Actions::new(vec![
        BlockElement::from(select),
        BlockElement::from(external),
        BlockElement::from(button),
        BlockElement::from(overflow),
    ])
    .block_id("actions_elements")
    .build()?;

    // Input blocks showcasing inputs/selectors
    let pt_input = Input::new(
        PlainText::new("Comments"),
        BlockElement::from(PlainTextInputElement::new("comment").placeholder(PlainText::new(
            "Add a comment",
        ))),
    )
    .build()?;

    let date_input = Input::new(
        PlainText::new("Date"),
        BlockElement::from(DatePickerElement::new("date")),
    )
    .build()?;

    let time_input = Input::new(
        PlainText::new("Time"),
        BlockElement::from(TimePickerElement::new("time")),
    )
    .build()?;

    let number_input = Input::new(
        PlainText::new("Quantity"),
        BlockElement::from(NumberInputElement::new("qty").min_value(0).max_value(10)),
    )
    .build()?;

    let email_input = Input::new(
        PlainText::new("Email"),
        BlockElement::from(EmailInputElement::new("email")),
    )
    .build()?;

    let url_input = Input::new(
        PlainText::new("URL"),
        BlockElement::from(UrlInputElement::new("url")),
    )
    .build()?;

    // Print each block as JSON for visual inspection
    for block in [
        actions,
        pt_input,
        date_input,
        time_input,
        number_input,
        email_input,
        url_input,
    ] {
        println!("{}", serde_json::to_string_pretty(&block.to_value()).unwrap());
    }

    Ok(())
}

