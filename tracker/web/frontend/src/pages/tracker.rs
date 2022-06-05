use crate::components::ClassCard;
use tracker_core::prelude::*;
use yew::prelude::*;

type CoreTracker = tracker_core::prelude::Tracker<Class>;

fn init_tracker() -> CoreTracker {
    const CLASS_PREFIX: &str = "CLASS";
    const N_CLASSES: u8 = 5;
    const N: u32 = 5;
    let gen = |a, b| Assignment::new(a, &format!("Assignment {b}"));

    let mut t = CoreTracker::new("Liam's Tracker");
    for i in 0..N_CLASSES {
        let letter = (b'A' + i) as char;
        let code = format!("{} {}", CLASS_PREFIX, letter);
        t.add_class(Class::new(&code)).unwrap();
        for j in 0..N {
            let id = j + (i as u32 * N);
            t.add_assignment(&code, gen(id, j)).unwrap();

            if j < N / 2 {
                t.get_assignment_mut(id)
                    .unwrap()
                    .set_mark(Mark::Letter(letter))
                    .unwrap();
            }
        }
    }

    t
}

#[function_component(Tracker)]
pub fn tracker() -> Html {
    let state = use_state_eq(init_tracker);
    let tracker = &*state;

    html! {
        <main class={classes!("tracker")}>
            <h1>{ "Liam's Tracker" }</h1>
            <div class={classes!("tracker_grid")}>
            {
                tracker.classes().iter().map(|class| {
                    html! {
                        <ClassCard
                            class={class.clone()}
                            assignments={
                                let mut assigns = tracker.assignments_from_class(class.code())
                                .iter()
                                .map(|&a| a.clone())
                                .collect::<Vec<_>>();

                                // use unstable sort since each name must be unique to each class
                                assigns.sort_unstable_by_key(|a| a.name().to_owned());
                                assigns
                            }
                        />
                    }
                }).collect::<Html>()
            }
            </div>
        </main>
    }
}
