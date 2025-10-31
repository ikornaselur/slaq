use slaq::blocks::elements::{
    button, channels_select, checkboxes, confirm, conversations_select, datepicker, datetimepicker,
    email_input, external_select, multi_channels_select, multi_conversations_select,
    multi_external_select, multi_users_select, multiselect, number_input, option, options,
    overflow, plain_text_input, radio_buttons, select, timepicker, url_input, users_select,
};
use slaq::blocks::text::{mrkdwn, plain};

#[test]
fn option_and_options_macros_match_builders() {
    let o1 = option!("A", "a");
    let o2 = option!("B", "b");
    let v1 = options![o1.clone(), o2.clone()];
    let v2 = vec![o1, o2];
    assert_eq!(
        serde_json::to_value(&v1).unwrap(),
        serde_json::to_value(&v2).unwrap()
    );
}

#[test]
fn confirm_macro_matches_builder() {
    let m = confirm!("Title", plain!("Body"), "OK", "Cancel");
    let b = slaq::blocks::ConfirmationDialog::new(
        slaq::blocks::PlainText::new("Title"),
        slaq::blocks::PlainText::new("Body"),
        slaq::blocks::PlainText::new("OK"),
        slaq::blocks::PlainText::new("Cancel"),
    );
    assert_eq!(
        serde_json::to_value(&m).unwrap(),
        serde_json::to_value(&b).unwrap()
    );
}

#[test]
fn button_macro_matches_builder() {
    let m = button!(
        "Click",
        "btn",
        url = "https://example.com",
        value = "v",
        style = slaq::blocks::ButtonStyle::Primary,
        confirm = confirm!("Are you sure?", mrkdwn!("Really?"), "Yes", "No"),
    );
    let b = slaq::blocks::ButtonElement::new(slaq::blocks::PlainText::new("Click"), "btn")
        .url("https://example.com")
        .value("v")
        .style(slaq::blocks::ButtonStyle::Primary)
        .confirm(slaq::blocks::ConfirmationDialog::new(
            slaq::blocks::PlainText::new("Are you sure?"),
            slaq::blocks::TextObject::from(mrkdwn!("Really?")),
            slaq::blocks::PlainText::new("Yes"),
            slaq::blocks::PlainText::new("No"),
        ));
    assert_eq!(
        serde_json::to_value(&m).unwrap(),
        serde_json::to_value(&b).unwrap()
    );
}

#[test]
fn select_macros_match_builders() {
    let m = select!(
        "sel",
        placeholder = "Pick",
        options = options![option!("A", "a"), option!("B", "b")],
        initial_option = option!("A", "a"),
    );
    let b = slaq::blocks::StaticSelectElement::new("sel")
        .placeholder(slaq::blocks::PlainText::new("Pick"))
        .options(vec![
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("A"), "a"),
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("B"), "b"),
        ])
        .initial_option(slaq::blocks::SelectOption::new(
            slaq::blocks::PlainText::new("A"),
            "a",
        ));
    assert_eq!(
        serde_json::to_value(&m).unwrap(),
        serde_json::to_value(&b).unwrap()
    );

    let mm = multiselect!(
        "msel",
        placeholder = "Pick",
        options = options![option!("A", "a")],
        initial_options = options![option!("A", "a")],
        max_selected_items = 1,
    );
    let mb = slaq::blocks::elements::MultiStaticSelectElement::new("msel")
        .placeholder(slaq::blocks::PlainText::new("Pick"))
        .options(vec![slaq::blocks::SelectOption::new(
            slaq::blocks::PlainText::new("A"),
            "a",
        )])
        .initial_options(vec![slaq::blocks::SelectOption::new(
            slaq::blocks::PlainText::new("A"),
            "a",
        )])
        .max_selected_items(1);
    assert_eq!(
        serde_json::to_value(&mm).unwrap(),
        serde_json::to_value(&mb).unwrap()
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn input_and_choice_macros_match_builders() {
    let dp = datepicker!("date", initial_date = "2025-01-01");
    let db = slaq::blocks::elements::DatePickerElement::new("date").initial_date("2025-01-01");
    assert_eq!(
        serde_json::to_value(&dp).unwrap(),
        serde_json::to_value(&db).unwrap()
    );

    let tp = timepicker!("time", initial_time = "13:37");
    let tb = slaq::blocks::elements::TimePickerElement::new("time").initial_time("13:37");
    assert_eq!(
        serde_json::to_value(&tp).unwrap(),
        serde_json::to_value(&tb).unwrap()
    );

    let dt = datetimepicker!("dt", initial_date_time = 1_700_000_000);
    let db2 =
        slaq::blocks::elements::DateTimePickerElement::new("dt").initial_date_time(1_700_000_000);
    assert_eq!(
        serde_json::to_value(&dt).unwrap(),
        serde_json::to_value(&db2).unwrap()
    );

    let macro_pt_input = plain_text_input!(
        "pti",
        placeholder = "Type here",
        min_length = 0,
        max_length = 10
    );
    let builder_pt_input = slaq::blocks::elements::PlainTextInputElement::new("pti")
        .placeholder(slaq::blocks::PlainText::new("Type here"))
        .min_length(0)
        .max_length(10);
    assert_eq!(
        serde_json::to_value(&macro_pt_input).unwrap(),
        serde_json::to_value(&builder_pt_input).unwrap()
    );

    let ei = email_input!("email", initial_value = "a@b.com");
    let eb = slaq::blocks::elements::EmailInputElement::new("email").initial_value("a@b.com");
    assert_eq!(
        serde_json::to_value(&ei).unwrap(),
        serde_json::to_value(&eb).unwrap()
    );

    let ui = url_input!("url", initial_value = "https://example.com");
    let ub =
        slaq::blocks::elements::UrlInputElement::new("url").initial_value("https://example.com");
    assert_eq!(
        serde_json::to_value(&ui).unwrap(),
        serde_json::to_value(&ub).unwrap()
    );

    let ni = number_input!("n", min_value = 0, max_value = 10, decimal_allowed = true);
    let nb = slaq::blocks::elements::NumberInputElement::new("n")
        .min_value(0)
        .max_value(10)
        .decimal_allowed(true);
    assert_eq!(
        serde_json::to_value(&ni).unwrap(),
        serde_json::to_value(&nb).unwrap()
    );

    let cb = checkboxes!(
        "checks",
        options = options![option!("A", "a"), option!("B", "b")],
        initial_options = options![option!("B", "b")],
    );
    let cbb = slaq::blocks::elements::CheckboxesElement::new(
        "checks",
        vec![
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("A"), "a"),
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("B"), "b"),
        ],
    )
    .initial_options(vec![slaq::blocks::SelectOption::new(
        slaq::blocks::PlainText::new("B"),
        "b",
    )]);
    assert_eq!(
        serde_json::to_value(&cb).unwrap(),
        serde_json::to_value(&cbb).unwrap()
    );

    let rb = radio_buttons!(
        "choice",
        options = options![option!("A", "a"), option!("B", "b")],
        initial_option = option!("A", "a"),
    );
    let rbb = slaq::blocks::elements::RadioButtonsElement::new(
        "choice",
        vec![
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("A"), "a"),
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("B"), "b"),
        ],
    )
    .initial_option(slaq::blocks::SelectOption::new(
        slaq::blocks::PlainText::new("A"),
        "a",
    ));
    assert_eq!(
        serde_json::to_value(&rb).unwrap(),
        serde_json::to_value(&rbb).unwrap()
    );

    let ov = overflow!(
        "more",
        options = options![option!("Edit", "edit"), option!("Delete", "delete")],
    );
    let ovb = slaq::blocks::elements::OverflowElement::new(
        "more",
        vec![
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("Edit"), "edit"),
            slaq::blocks::SelectOption::new(slaq::blocks::PlainText::new("Delete"), "delete"),
        ],
    );
    assert_eq!(
        serde_json::to_value(&ov).unwrap(),
        serde_json::to_value(&ovb).unwrap()
    );
}

#[test]
fn identity_select_macros_match_builders() {
    let us = users_select!("u", initial_user = "U1");
    let usb = slaq::blocks::elements::UsersSelectElement::new("u").initial_user("U1");
    assert_eq!(
        serde_json::to_value(&us).unwrap(),
        serde_json::to_value(&usb).unwrap()
    );

    let macro_multi_users = multi_users_select!(
        "mu",
        initial_users = vec!["U1".into(), "U2".into()],
        max_selected_items = 2
    );
    let builder_multi_users = slaq::blocks::elements::MultiUsersSelectElement::new("mu")
        .initial_users(vec!["U1".into(), "U2".into()])
        .max_selected_items(2);
    assert_eq!(
        serde_json::to_value(&macro_multi_users).unwrap(),
        serde_json::to_value(&builder_multi_users).unwrap()
    );

    let macro_convs = conversations_select!("c", default_to_current_conversation = true);
    let builder_convs = slaq::blocks::elements::ConversationsSelectElement::new("c")
        .default_to_current_conversation(true);
    assert_eq!(
        serde_json::to_value(&macro_convs).unwrap(),
        serde_json::to_value(&builder_convs).unwrap()
    );

    let macro_multi_convs =
        multi_conversations_select!("mc", initial_conversations = vec!["C1".into()]);
    let builder_multi_convs = slaq::blocks::elements::MultiConversationsSelectElement::new("mc")
        .initial_conversations(vec!["C1".into()]);
    assert_eq!(
        serde_json::to_value(&macro_multi_convs).unwrap(),
        serde_json::to_value(&builder_multi_convs).unwrap()
    );

    let macro_channels = channels_select!("ch", initial_channel = "C1");
    let builder_channels =
        slaq::blocks::elements::ChannelsSelectElement::new("ch").initial_channel("C1");
    assert_eq!(
        serde_json::to_value(&macro_channels).unwrap(),
        serde_json::to_value(&builder_channels).unwrap()
    );

    let macro_multi_channels =
        multi_channels_select!("mch", initial_channels = vec!["C1".into(), "C2".into()]);
    let builder_multi_channels = slaq::blocks::elements::MultiChannelsSelectElement::new("mch")
        .initial_channels(vec!["C1".into(), "C2".into()]);
    assert_eq!(
        serde_json::to_value(&macro_multi_channels).unwrap(),
        serde_json::to_value(&builder_multi_channels).unwrap()
    );

    let macro_external_select = external_select!("ex", min_query_length = 2);
    let builder_external_select =
        slaq::blocks::elements::ExternalSelectElement::new("ex").min_query_length(2);
    assert_eq!(
        serde_json::to_value(&macro_external_select).unwrap(),
        serde_json::to_value(&builder_external_select).unwrap()
    );

    let macro_multi_external_select = multi_external_select!("mex", max_selected_items = 3);
    let builder_multi_external_select =
        slaq::blocks::elements::MultiExternalSelectElement::new("mex").max_selected_items(3);
    assert_eq!(
        serde_json::to_value(&macro_multi_external_select).unwrap(),
        serde_json::to_value(&builder_multi_external_select).unwrap()
    );
}
