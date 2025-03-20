use leptos::*;

#[derive(Clone)]
pub struct StatData {
    pub title: String,
    pub value: String,
    pub description: Option<String>,
}

#[component]
pub fn StatCard(stat: StatData) -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground rounded-lg p-4 shadow-sm">
            <h3 class="text-sm font-medium text-muted-foreground">{stat.title}</h3>
            <p class="text-2xl font-bold mt-1">{stat.value}</p>
            {
                move || stat.description.as_ref().map(|desc| view! {
                    <p class="text-xs text-muted-foreground mt-1">{desc}</p>
                })
            }
        </div>
    }
}
