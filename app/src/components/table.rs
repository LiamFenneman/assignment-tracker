use leptos::*;
use tracker_core::*;

#[component]
pub fn CourseTable(cx: Scope, course: Course) -> impl IntoView {
    let (assigns, _) = create_signal(cx, course.assignments);

    view! {
        cx,
        <div class="flex flex-col">
            <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div class="py-2 inline-block min-w-full sm:px-6 lg:px-8">
                    <div class="overflow-hidden">
                        <table class="min-w-full">
                            <thead class="bg-slate-100 border-b">
                                <tr>
                                    <th class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        "Name"
                                    </th>
                                    <th class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        "Mark"
                                    </th>
                                    <th class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        "Weight"
                                    </th>
                                    <th class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        "Percentage"
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                            <For
                                each=assigns
                                key=|a: &Assignment| a.name().to_owned()
                                view=move |a: Assignment| view! { cx, <TableElement assignment=a /> }
                            />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TableElement(cx: Scope, assignment: Assignment) -> impl IntoView {
    view! {
        cx,
        <tr class="odd:bg-white even:bg-slate-50 border-b transition duration-300 ease-in-out hover:bg-gray-100">
            <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                {assignment.name().to_owned()}
            </td>
            <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                {format!("{:?}", assignment.mark())}
            </td>
            <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                {format!("{:?}", assignment.weight())}
            </td>
            <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                {format!("{:?}", assignment.percentage())}
            </td>
        </tr>
    }
}
