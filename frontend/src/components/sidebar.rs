use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar(
    #[prop(into)] is_open: Signal<bool>,
) -> impl IntoView {
    let current_route = use_route();
    
    let is_active = move |path: &str| {
        current_route().path() == path
    };
    
    view! {
        <aside 
            class=move || format!(
                "bg-background border-r border-border h-screen transition-all duration-300 {} overflow-y-auto",
                if is_open() { "w-64" } else { "w-0" }
            )
        >
            <div class="p-4">
                <h1 class="text-xl font-bold mb-6">Slynqix</h1>
                
                <nav class="space-y-2">
                    <SidebarLink 
                        path="/" 
                        label="Dashboard" 
                        is_active=is_active("/")
                    />
                    <SidebarLink 
                        path="/console" 
                        label="Console" 
                        is_active=is_active("/console")
                    />
                    <SidebarLink 
                        path="/aftermarket-analyzer" 
                        label="Aftermarket Analyzer" 
                        is_active=is_active("/aftermarket-analyzer")
                    />
                    <SidebarLink 
                        path="/history" 
                        label="History" 
                        is_active=is_active("/history")
                    />
                    <SidebarLink 
                        path="/mindsage" 
                        label="Mindsage" 
                        is_active=is_active("/mindsage")
                    />
                    <SidebarLink 
                        path="/algo-trading" 
                        label="Algo Trading" 
                        is_active=is_active("/algo-trading")
                    />
                    <SidebarLink 
                        path="/global-sentiment" 
                        label="Global Sentiment" 
                        is_active=is_active("/global-sentiment")
                    />
                    <SidebarLink 
                        path="/model-trainer" 
                        label="Model Trainer" 
                        is_active=is_active("/model-trainer")
                    />
                    <SidebarLink 
                        path="/journal" 
                        label="Journal" 
                        is_active=is_active("/journal")
                    />
                    <SidebarLink 
                        path="/profile" 
                        label="Profile" 
                        is_active=is_active("/profile")
                    />
                </nav>
            </div>
        </aside>
    }
}

#[component]
fn SidebarLink(
    path: &'static str,
    label: &'static str,
    #[prop(into)] is_active: Signal<bool>,
) -> impl IntoView {
    view! {
        <A 
            href=path
            class=move || format!(
                "flex items-center px-3 py-2 rounded hover:bg-secondary transition-colors {}",
                if is_active() { "bg-primary text-primary-foreground" } else { "" }
            )
        >
            <span>{label}</span>
        </A>
    }
}
