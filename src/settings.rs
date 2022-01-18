use yew::prelude::*;
use crate::{glider_selector::GliderSelector, App};

pub enum Msg {
    Confirm,
}

#[derive(Properties, Clone)]
pub struct SettingsProps {
    pub app_link: yew::html::Scope<App>,
}

impl PartialEq for SettingsProps {
    fn eq(&self, _other: &Self) -> bool { true }
}

pub struct Settings {}

impl Component for Settings {
    type Message = Msg;
    type Properties = SettingsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Confirm => {
                ctx.props().app_link.send_message(crate::Msg::SetPage(crate::Page::Agenda));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<>
            <header>
                <h1>{"Paramètres"}</h1>
            </header>
            <main id="settings-main">
                <h2>{"Paramètres du compte"}</h2>
                <div class="settings-group">
                    <div class="setting">
                        <h3>{"Mot de passe"}</h3>
                        <p>{"Votre mot de passe a été changé pour la dernière fois le 12/11/2021 à 12:49."}</p>
                        <div class="white-button small-button">{"Modifier"}</div>
                    </div>
                    <br/>
                    <br/>
                    <div class="setting">
                        <h3>{"Adresse mail"}</h3>
                        <p>{"Votre adresse actuelle est foobar@insa-rouen.fr."}</p>
                        <div class="white-button small-button">{"Modifier"}</div>
                    </div>
                    <br/>
                    <br/>
                    <div class="setting">
                        <h3>{"Changer le type d'authentification"}</h3>
                        <p>{"L'authentification par email consiste a rentrer un code unique qui vous sera envoyé par email."}</p>
                        <GliderSelector values={vec!["Email", "Mot de passe", "Email + Mot de passe"]} />
                    </div>
                </div>
                <h2>{"Affichage"}</h2>
                <div class="settings-group">
                    <div class="setting">
                        <h3>{"Thème"}</h3>
                        <p>{"Par défault, le thème est celui renseigné par votre navigateur."}</p>
                        <GliderSelector values={vec!["Automatique", "Sombre", "Clair"]} />
                    </div>
                    <br/>
                    <br/>
                    <div class="setting">
                        <h3>{"Nom des bâtiments"}</h3>
                        <p>{"L'affichage court correspond à seulement les deux premières lettres du nom (ex: Ma plutôt que Magellan)."}</p>
                        <GliderSelector values={vec!["Normal", "Court"]} />
                    </div>
                </div>
                <div class="red-button" onclick={ctx.link().callback(move |_| Msg::Confirm)}>{"Valider"}</div>
            </main>
            <footer>
            </footer>
        </>}
    }
}
