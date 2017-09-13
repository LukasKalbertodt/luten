use maud::{html, Markup};


use super::StudentPreferences;
use dict::{self, Locale};
use user::Student;
use timeslot::{Rating, TimeSlot};



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

pub fn student_timeslots(slots: &[TimeSlot], locale: Locale) -> Markup {
    html! {
        div class="c-card prep-status-card u-higher" {
            div class="c-card__item c-card__item--info c-card__item--divider" {
                "Erklärbär"
            }
            div class="c-card__item" {
                "Also, hör mal zu, ich erklär dir mal wie das geht."
            }
        }

        h1 class="c-heading" "Zeitslots"

        form action="/prep/update_timeslots" method="post" {
            (timeslot_list(slots, locale, timeslot_rating))

            input
                class="c-button c-button--success u-large c-button--block"
                type="submit"
                {}
        }
    }
}

pub fn timeslot_list<F>(slots: &[TimeSlot], locale: Locale, mut slot_formatter: F) -> Markup
    where F: FnMut(Option<TimeSlot>, Rating, Locale) -> Markup
{
    use std::collections::BTreeMap;

    let mut days = BTreeMap::new();
    for slot in slots {
        days.entry(slot.day())
            .or_insert(Vec::new())
            .push(Some(slot));
    }

    for v in days.values_mut().filter(|v| !v.is_empty()) {
        v.sort();

        let mut last_time = v[0].unwrap().time();
        let mut i = 1;
        while i < v.len() {
            if v[i].unwrap().time().prev() != last_time {
                v.insert(i, None);
            }
            last_time = last_time.next();

            i += 1;
        }
    }

    html! {
        div class="timeslots-grid" {
            @for (day, slots) in days {
                div class="timeslots-grid-cell" {
                    h3 class="heading" (day.full_name(locale))
                    @for slot in slots {
                        (slot_formatter(slot.cloned(), Rating::Bad, locale))
                    }
                }
            }
        }
    }
}

pub fn timeslot_rating(slot: Option<TimeSlot>, rating: Rating, _locale: Locale) -> Markup {
    if let Some(slot) = slot {
        let name = format!("slot-{}", slot.id());
        let id_good = format!("slot-pref-{}-good", slot.id());
        let id_tolerable = format!("slot-pref-{}-tolerable", slot.id());
        let id_bad = format!("slot-pref-{}-bad", slot.id());

        html! {
            div class="c-button-group--rounded timeslots-slot" {
                label (slot.time())

                input
                    type="radio"
                    name=(name)
                    id=(id_good)
                    value="good"
                    class="timeslots-rating"
                    checked?[rating == Rating::Good];
                label for=(id_good) class="c-button c-button--ghost-success" "Passt"

                input
                    type="radio"
                    name=(name)
                    id=(id_tolerable)
                    value="tolerable"
                    class="timeslots-rating"
                    checked?[rating == Rating::Tolerable];
                label for=(id_tolerable) class="c-button c-button--ghost-warning" "Ungern"

                input
                    type="radio"
                    name=(name)
                    id=(id_bad)
                    value="bad"
                    class="timeslots-rating"
                    checked?[rating == Rating::Bad];
                label for=(id_bad) class="c-button c-button--ghost-error" "Passt nicht"
            }
        }
    } else {
        html! {
            div class="timeslots-empty-slot" {
                i class="fa fa-ban" {}
            }
        }
    }
}