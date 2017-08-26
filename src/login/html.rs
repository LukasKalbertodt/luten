use maud::{html, Markup};

use dict::{self, Locale};
use login::ProviderEntry;

pub fn login_page(providers: &[&ProviderEntry], locale: Locale) -> Markup {
    let dict = dict::new(locale).login;
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
                    div class="c-card__item c-card__item--brand c-card__item--divider" (dict.box_title())
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
                                        placeholder=(dict.username_placeholder());
                                    div class="c-hint" (dict.username_hint())
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
                                        placeholder=(dict.password_placeholder());
                                    div class="c-hint" (dict.password_hint())
                                }
                            }

                            // Login provider drop-down menu. It is hidden if
                            // there is only one provider.
                            div
                                class="o-form-element"
                                style=(if providers.len() == 1 { "display:none " } else { "" })
                            {
                                label class="c-label" for="login_provider" (dict.login_provider_label())
                                select class="c-field" name="login_provider" {
                                    @for provider in providers {
                                        // TODO: l10n
                                        option value=(provider.id) (provider.imp.name())
                                    }
                                }
                            }

                            // Button: "Login"
                            div class="u-letter-box--small" {
                                input
                                    type="submit"
                                    value=(dict.login_button_label())
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
                    div class="c-card__item c-card__item--divider" (dict.notice_box_title())
                    div class="c-card__item" {
                        // TODO: i10n
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
