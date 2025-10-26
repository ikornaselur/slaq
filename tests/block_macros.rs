use slaq::{divider, header, mrkdwn, plain, section, video};

#[test]
fn header_macro_matches_builder() {
    let macro_block = header!("Status").expect("header");
    let builder_block = slaq::blocks::Header::new(plain!("Status")).build().unwrap();
    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn divider_macro_matches_builder() {
    let macro_block = divider!().unwrap();
    let builder_block = slaq::blocks::Divider::new().build().unwrap();
    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn video_macro_matches_builder() {
    let macro_block = video!(
        title = "Highlights",
        video_url = "https://example.com/embed/abc",
        thumbnail_url = "https://example.com/thumb.jpg",
        alt_text = "Product demo",
        description = "Slack sure is nifty!",
        provider_name = "ExampleTube",
    )
    .unwrap();

    let builder_block = slaq::blocks::Video::new(
        plain!("Highlights"),
        "https://example.com/embed/abc",
        "https://example.com/thumb.jpg",
        "Product demo",
    )
    .description(plain!("Slack sure is nifty!"))
    .provider_name("ExampleTube")
    .build()
    .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}

#[test]
fn section_macro_matches_builder() {
    let macro_block = section!(
        text = mrkdwn!("*Hello*"),
        fields = vec![mrkdwn!("Field1").into(), mrkdwn!("Field2").into()],
    )
    .unwrap();

    let builder_block = slaq::blocks::Section::new()
        .text(mrkdwn!("*Hello*"))
        .fields(vec![mrkdwn!("Field1").into(), mrkdwn!("Field2").into()])
        .build()
        .unwrap();

    assert_eq!(macro_block.to_value(), builder_block.to_value());
}
