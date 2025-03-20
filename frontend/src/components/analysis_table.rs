use leptos::*;

#[derive(Clone)]
pub struct AnalysisRow {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[component]
pub fn AnalysisTable(
    title: &'static str,
    rows: Vec<AnalysisRow>,
) -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground rounded-lg shadow-sm overflow-hidden">
            <div class="p-4 border-b border-border">
                <h3 class="text-lg font-medium">{title}</h3>
            </div>
            <div class="p-0">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-border">
                            <th class="text-left p-3 text-muted-foreground font-medium">Metric</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Value</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Description</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            rows.into_iter().map(|row| {
                                view! {
                                    <tr class="border-b border-border">
                                        <td class="p-3">{row.name}</td>
                                        <td class="p-3 font-medium">{row.value}</td>
                                        <td class="p-3 text-muted-foreground">
                                            {row.description.unwrap_or_default()}
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
