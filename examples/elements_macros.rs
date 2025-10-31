use slaq::blocks::elements::{
    button, datepicker, external_select, option, options, overflow, select, timepicker,
};
use slaq::blocks::text::mrkdwn;
use slaq::blocks::{self, section};

fn main() -> Result<(), slaq::blocks::BuildError> {
    let blocks = blocks::blocks_vec![
        section!(
            text = mrkdwn!("Pick one from static options"),
            accessory = select!(
                "sel_1",
                placeholder = "Pick",
                options = options![option!("A", "a"), option!("B", "b")],
                initial_option = option!("A", "a"),
            ),
        ),
        section!(
            text = mrkdwn!("Or search externally"),
            accessory =
                external_select!("ex_1", placeholder = "Type to search", min_query_length = 2,),
        ),
        section!(
            text = mrkdwn!("Take action"),
            accessory = button!("Confirm", "btn_confirm"),
        ),
        section!(
            text = mrkdwn!("More options"),
            accessory = overflow!(
                "more",
                options = options![option!("Edit", "edit"), option!("Delete", "delete")],
            ),
        ),
        section!(
            text = mrkdwn!("Pick a date"),
            accessory = datepicker!("date_1"),
        ),
        section!(
            text = mrkdwn!("Pick a time"),
            accessory = timepicker!("time_1"),
        ),
    ]?;

    for block in blocks {
        println!(
            "{},",
            serde_json::to_string_pretty(&block.to_value()).unwrap()
        );
    }

    Ok(())
}
