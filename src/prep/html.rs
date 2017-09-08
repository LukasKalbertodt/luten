use maud::{html, Markup};


use dict::{self, Locale};



pub fn student_overview(locale: Locale) -> Markup {
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

        h2 "Einstellungen"
        form method="post" action="/prep_student_settings" {
            section {
                h3 class="c-heading" "Partner"
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed flex-right c-card" {
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
                                input type="radio" name="partner" value="random" "Zufallspartner"
                            }
                            label class="c-field c-field--choice" {
                                input type="radio" name="partner" value="chosen" "Partner auswählen"
                                br;
                                input type="text" name="partner_id" {}
                            }
                        }
                    }
                }
            }

            section {
                h3 class="c-heading" "Bevorzugte Sprache"
                div class="o-grid o-grid--xsmall-full o-grid--small-full o-grid--medium-full" {
                    div class="o-grid__cell o-grid__cell--width-fixed flex-right c-card" {
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
                                input type="radio" name="language" value="de" checked? "Deutsch"
                            }
                            label class="c-field c-field--choice" {
                                input type="radio" name="language" value="en" "English"
                            }
                        }
                    }
                }
            }

            input type="submit" "Speichern"
        }
    }
}
