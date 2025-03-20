use leptos::*;
use leptos_router::*;

#[component]
pub fn ComingSoon(title: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-full">
            <h1 class="text-3xl font-bold mb-4">{title}</h1>
            <p class="text-lg text-muted-foreground mb-8">Coming Soon...</p>
            <A 
                href="/" 
                class="px-6 py-3 bg-primary text-primary-foreground rounded-lg shadow hover:bg-primary/90 transition-colors"
            >
                Explore Slynqix
            </A>
        </div>
    }
}
