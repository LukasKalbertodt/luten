use maud::{html, Markup};

use login::ProviderEntry;

pub fn login_page(providers: &[&ProviderEntry]) -> Markup {
    html! {
        div class="o-grid o-grid--small-full u-letter-box--small" {
            // Empty grid cell to have a blank buffer on the side
            div class="o-grid__cell o-grid__cell--no-gutter" {}

            // Actual login-box
            div
                class="o-grid__cell o-grid__cell--width-fixed u-letter-box--small"
                style="width: 25em"
            {
                div class="c-card" {
                    div class="c-card__item c-card__item--brand c-card__item--divider" "Einloggen"
                    div class="c-card__item" {
                        form method="post" action="/login" class="u-pillar-box--small" {
                            // Field: "username"
                            div class="u-letter-box--small login-field-container u-large" {
                                div class="o-field o-field--icon-left" {
                                    i class="fa fa-fw fa-user c-icon" {}
                                    input
                                        type="text"
                                        class="c-field"
                                        name="id"
                                        placeholder="Nutzerkennung";
                                    div class="c-hint" {
                                        "Deine RZ-Kennung (z.B. "
                                        i "xmuster"
                                        ")"
                                    }
                                }
                            }

                            // Field: "password"
                            div class="u-letter-box--small login-field-container u-large" {
                                div class="o-field o-field--icon-left" {
                                    i class="fa fa-fw fa-key c-icon" {}
                                    input
                                        type="password"
                                        class="c-field"
                                        name="secret"
                                        placeholder="Password";
                                    div class="c-hint" "Dein globales Uni-Password"
                                }
                            }

                            // Login provider drop-down menu. It is hidden if
                            // there is only one provider.
                            div
                                class="o-form-element"
                                style=(if providers.len() == 1 { "display:none " } else { "" })
                            {
                                label class="c-label" for="login_provider" "Anmelden durch:"
                                select class="c-field" name="login_provider" {
                                    @for provider in providers {
                                        option value=(provider.id) (provider.imp.name())
                                    }
                                }
                            }

                            // Button: "Login"
                            div class="u-letter-box--small" {
                                input
                                    type="submit"
                                    value="Login"
                                    class="c-button c-button--success"
                                    style="width: 100%";
                            }
                        }
                    }
                }
            }

            // Notice box
            div
                class="o-grid__cell o-grid__cell--width-fixed u-letter-box--small"
                style="width: 30em"
            {
                div class="c-card" {
                    div class="c-card__item c-card__item--divider" "Hinweise"
                    div class="c-card__item" {
                        ul class="login-notice-list" {
                            li "Nutze zum Einloggen deine normalen Uni-Login-Daten"
                            li "Die Authentifizierung erfolgt über das Uni-LDAP"
                            li "Dein Password wird zu keiner Zeit gespeichert"
                        }
                    }
                }
            }

            // Empty grid cell to have a blank buffer on the side
            div class="o-grid__cell o-grid__cell--no-gutter" {}
        }
    }
}
