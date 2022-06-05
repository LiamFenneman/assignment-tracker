use std::fmt::Display;
use tracker_core::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<C = Class, A = Assignment>
where
    C: Classlike,
    A: Assignmentlike,
{
    pub class: C,
    #[prop_or_default]
    pub assignments: Vec<A>,
}

pub struct ClassCard<C = Class, A = Assignment>
where
    C: Classlike,
    A: Assignmentlike,
{
    class: C,
    assignments: Vec<A>,
}

impl Component for ClassCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            class: props.class.clone(),
            assignments: props.assignments.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("card")}>
                <h2>{ self.class.name() }</h2>
                <table>
                    <tr>
                        <th>{ "Assignment" }</th>
                        <th>{ "Mark" }</th>
                        <th>{ "Value" }</th>
                        <th>{ "Due Date" }</th>
                        <th>{ "Status" }</th>
                    </tr>
                    {
                        self.assignments.iter().map(|assignment| {
                            html! {
                                <tr>
                                    <td>{ assignment.name() }</td>
                                    <td>{ option_to_string(assignment.mark()) }</td>
                                    <td>{ assignment.value() }</td>
                                    <td>{ option_to_string(assignment.due_date()) }</td>
                                    // TODO: replace with `status` method
                                    <td>{ match assignment.mark() {
                                        Some(_) => "Complete",
                                        None => "Incomplete",
                                    } }</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </table>
            </div>
        }
    }
}

fn option_to_string<T: Display>(opt: Option<T>) -> String {
    match opt {
        Some(val) => val.to_string(),
        None => "None".to_string(),
    }
}
