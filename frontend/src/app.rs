use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::pages::dashboard::Dashboard;
use crate::pages::console::Console;
use crate::pages::aftermarket_analyzer::AftermarketAnalyzer;
use crate::pages::history::History;
use crate::pages::mindsage::Mindsage;
use crate::pages::algo_trading::AlgoTrading;
use crate::pages::global_sentiment::GlobalSentiment;
use crate::pages::model_trainer::ModelTrainer;
use crate::pages::journal::Journal;
use crate::pages::profile::Profile;
use crate::utils::theme::ThemeProvider;

#[component]
pub fn App() -> impl IntoView {
    let (is_sidebar_open, set_sidebar_open) = create_signal(true);
    
    let toggle_sidebar = move |_| {
        set_sidebar_open.update(|open| *open = !*open);
    };
    
    view! {
        <ThemeProvider>
            <Router>
                <div class="flex h-screen">
                    <Sidebar is_open=is_sidebar_open />
                    <div class="flex flex-col flex-grow overflow-hidden">
                        <Header toggle_sidebar=toggle_sidebar />
                        <main class="flex-grow overflow-auto p-6">
                            <Routes>
                                <Route path="/" view=|| view! { <Dashboard /> } />
                                <Route path="/console" view=|| view! { <Console /> } />
                                <Route path="/aftermarket-analyzer" view=|| view! { <AftermarketAnalyzer /> } />
                                <Route path="/history" view=|| view! { <History /> } />
                                <Route path="/mindsage" view=|| view! { <Mindsage /> } />
                                <Route path="/algo-trading" view=|| view! { <AlgoTrading /> } />
                                <Route path="/global-sentiment" view=|| view! { <GlobalSentiment /> } />
                                <Route path="/model-trainer" view=|| view! { <ModelTrainer /> } />
                                <Route path="/journal" view=|| view! { <Journal /> } />
                                <Route path="/profile" view=|| view! { <Profile /> } />
                                <Route path="/*" view=|| view! { <div>"Not Found"</div> } />
                            </Routes>
                        </main>
                    </div>
                </div>
            </Router>
        </ThemeProvider>
    }
}
