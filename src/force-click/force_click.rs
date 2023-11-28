use crate::prelude::*;
use chrono::Local;

const KEY: &str = "last_click_date";

pub struct ForceClickComp {
    last_click: Option<i32>,
}

impl Component for ForceClickComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let last_click = {
            let local_storage = window().local_storage().unwrap().unwrap();
            local_storage.get(KEY).unwrap_or_default().map(|date| date.parse::<i32>().unwrap_or_default())
        };

        Self {
            last_click,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let today = Local::now().naive_local().num_days_from_ce();
        let local_storage = window().local_storage().unwrap().unwrap();
        local_storage.set(KEY, &today.to_string()).unwrap();
        self.last_click = Some(today);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let today = Local::now().naive_local().num_days_from_ce();
        let show_popup = !self.last_click.map_or(false, |date| date == today);

        if show_popup {
            template_html!(
                "src/force-click/force_click.html",
                show_popup = show_popup,
                onclick_link = { ctx.link().callback(|_| ()) },
            )
        } else {
            html! {
                <div></div>
            }
        }
    }
}