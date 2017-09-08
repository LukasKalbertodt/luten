use maud::{html, Markup};


use super::StudentPreferences;
use dict::{self, Locale};



pub fn student_overview(locale: Locale, pref: &StudentPreferences) -> Markup {
    // TODO: l10n
    let dict = dict::new(locale).prep;

    html! {
        div class="c-card prep-status-card u-higher" {
            div class="c-card__item c-card__item--info c-card__item--divider" {
                (dict.state_title())
                ": Informationen"
            }
            div class="c-card__item" {
                p (dict.explanation())
            }
            div class="c-card__item" {
                b "Dein Status:"
                ul {
                    li "Terminwahl: Keine Termine ausgewählt!"
                    li "Partner: Zufallspartner"
                    li "Sprache: Deutsch"
                }
            }
        }

        h1 class="c-heading" "Einstellungen"
        form method="post" action="/prep_student_settings" {
            section class="u-letter-box--medium" {
                h2 class="c-heading" "Partner"
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed prep-hint-box c-card" {
                        div class="c-card__item c-card__item--info c-card__item--divider" {
                            "Hinweise"
                        }
                        div class="c-card__item" {
                            "Du kannst dir entweder einen Zufallspartner zuweisen lassen oder einen "
                            "Kommilitonen angeben, den du gerne als Partner hättest."
                        }
                    }
                    div class="o-grid__cell o-grid__cell--width-60" {
                        fieldset class="o-fieldset" name="partner" {
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    name="partner"
                                    value="random"
                                    checked?[pref.partner.is_none()]
                                    "Zufallspartner"
                            }
                            label class="c-field c-field--choice" {
                                input
                                    type="radio"
                                    name="partner"
                                    value="chosen"
                                    checked?[pref.partner.is_some()]
                                    "Partner auswählen"
                                br;
                                input type="text" name="partner_id" {}
                            }
                        }
                    }
                }
            }

            section class="u-letter-box--medium" {
                h2 class="c-heading" "Bevorzugte Sprache"
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed prep-hint-box c-card" {
                        div class="c-card__item c-card__item--info c-card__item--divider" {
                            "Hinweise:"
                        }
                        div class="c-card__item" {
                            "Wenn du der Deutschen Sprache nicht mächtig bist, kannst du hier festlegen, "
                            "dass du lieber ein Testat auf Englisch möchtest."

                        }
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
                            value="Speichern" {}
                    }
                }
            }
        }
    }
}
