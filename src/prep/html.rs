use maud::{html, Markup};


use super::StudentPreferences;
use dict::{self, Locale};
use user::Student;



pub fn student_overview(
    locale: Locale,
    pref: &StudentPreferences,
    partner: &Option<Student>
) -> Markup {
    // TODO: l10n
    let dict = dict::new(locale).prep;

    html! {
        div class="c-card prep-status-card u-higher" {
            div class="c-card__item c-card__item--info c-card__item--divider" {
                (dict.explanation_box_title())
            }
            div class="c-card__item" {
                p (dict.explanation_for_students())
            }
            div class="c-card__item" ({
                // Name of partner or random partner
                let partner = if let Some(ref partner_id) = pref.partner {
                    html! {
                        (partner_id)
                        @if let Some(ref name) = partner.as_ref().and_then(|s| s.name()) {
                            " (" (name) ")"
                        }
                    }
                } else {
                    html! { i (dict.random_partner()) }
                };

                // Language
                let language = if pref.prefers_english {
                    "English"
                } else {
                    "Deutsch"
                };

                dict.student_status(
                    html! { "Keine Termine ausgewählt" },   // TODO
                    html! { b (partner) },
                    html! { b (language) },
                )
            })
        }

        h1 class="c-heading" (dict.settings_headline())
        form method="post" action="/prep_student_settings" {
            section class="u-letter-box--medium" {
                h2 class="c-heading" (dict.partner_sub_headline())
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed prep-hint-box c-card" {
                        div class="c-card__item c-card__item--info c-card__item--divider" {
                            (dict.hints_title())
                        }
                        div class="c-card__item" (dict.partner_hints())
                    }
                    div class="o-grid__cell o-grid__cell--width-60" {
                        fieldset class="o-fieldset" name="partner" {
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    name="partner"
                                    value="random"
                                    checked?[pref.partner.is_none()]
                                    onchange="Luten.Util.disableField('prep-partner-field')"
                                    (dict.random_partner())
                            }
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    class="prep-partner-chosen"
                                    name="partner"
                                    value="chosen"
                                    checked?[pref.partner.is_some()]
                                    onchange="Luten.Util.enableField('prep-partner-field')"
                                    (dict.choose_partner())

                                div class="u-letter-box--small prep-partner-field-container" {
                                    div class="o-field o-field--icon-left" {
                                        div class="c-icon" {
                                            i class={
                                                "fa fa-fw "
                                                @if partner.is_some() {
                                                    "fa-check-square-o"
                                                } @else if pref.partner.is_some() {
                                                    "fa-exclamation-triangle"
                                                } @else {
                                                    "fa-user"
                                                }
                                            } {}
                                        }

                                        input
                                            type="text"
                                            id="prep-partner-field"
                                            class="c-field prep-partner-field"
                                            name="partner_id"
                                            placeholder=(dict.id_of_partner_placeholder())
                                            value=(pref.partner.as_ref().map(|s| s.as_str()).unwrap_or(""))
                                            oninput="Luten.Prep.checkPartner(this)"
                                            disabled?[pref.partner.is_none()]
                                            {}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section class="u-letter-box--medium" {
                h2 class="c-heading" (dict.language_sub_headline())
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed prep-hint-box c-card" {
                        div class="c-card__item c-card__item--info c-card__item--divider" {
                            (dict.hints_title())
                        }
                        div class="c-card__item" (dict.language_hints())
                    }
                    div class="o-grid__cell o-grid__cell--width-60" {
                        fieldset class="o-fieldset" {
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    name="language"
                                    value="de"
                                    checked?[!pref.prefers_english]
                                    "Deutsch"
                            }
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    name="language"
                                    value="en"
                                    checked?[pref.prefers_english]
                                    "English"
                            }
                        }
                    }
                }
            }

            section class="u-letter-box--medium" {
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed prep-hint-box" {
                    }
                    div class="o-grid__cell o-grid__cell--width-60" {
                        input
                            class="c-button c-button--success u-large c-button--block"
                            type="submit"
                            value=(dict.save_form()) {}
                    }
                }
            }
        }
    }
}
