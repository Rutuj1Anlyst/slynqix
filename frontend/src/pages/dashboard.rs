use leptos::*;
use leptos_router::*;

use crate::components::market_card::*;
use crate::utils::api::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    // Check if user is logged in (simulated)
    let (is_logged_in, set_logged_in) = create_signal(false);
    
    // Mock market data
    let (market_data, set_market_data) = create_signal(vec![
        MarketData {
            symbol: "Nifty 50".to_string(),
            price: 24587.25,
            change: 123.45,
            change_percent: 0.51,
        },
        MarketData {
            symbol: "BankNifty".to_string(),
            price: 47325.80,
            change: -87.65,
            change_percent: -0.18,
        },
        MarketData {
            symbol: "FinNifty".to_string(),
            price: 22631.50,
            change: 45.20,
            change_percent: 0.2,
        },
        MarketData {
            symbol: "Sensex".to_string(),
            price: 80975.10,
            change: 312.30,
            change_percent: 0.39,
        },
        MarketData {
            symbol: "Midcap".to_string(),
            price: 46875.60,
            change: -156.40,
            change_percent: -0.33,
        },
    ]);
    
    let handle_login = move |_| {
        set_logged_in.set(true);
    };
    
    view! {
        <div>
            {move || if !is_logged_in() {
                view! {
                    <div class="max-w-4xl mx-auto py-10">
                        <h1 class="text-3xl font-bold mb-6">Welcome to Slynqix</h1>
                        <p class="text-lg text-muted-foreground mb-8">
                            A comprehensive market analysis platform for trading and investment suggestions
                        </p>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
                            <FeatureCard 
                                title="Console"
                                description="Analyze market data and get trading suggestions"
                            />
                            <FeatureCard 
                                title="Journal"
                                description="Track your trades and analyze performance"
                            />
                            <FeatureCard 
                                title="Mindsage"
                                description="Get investment guidance based on fundamentals and technicals"
                            />
                            <FeatureCard 
                                title="Algo Trading"
                                description="Automate trades using algorithms or voice commands"
                            />
                        </div>
                        
                        <div class="flex justify-center">
                            <button 
                                class="px-6 py-3 bg-primary text-primary-foreground rounded-lg shadow hover:bg-primary/90 transition-colors"
                                on:click=handle_login
                            >
                                Log In
                            </button>
                        </div>
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="text-2xl font-bold mb-6">Market Overview</h1>
                        
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-4 mb-8">
                            {market_data.get().into_iter().map(|data| view! {
                                <MarketCard market=data />
                            }).collect::<Vec<_>>()}
                        </div>
                        
                        <h2 class="text-xl font-bold mb-4">Quick Access</h2>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <QuickAccessCard 
                                title="Console"
                                description="Analyze market data and get trading suggestions"
                                path="/console"
                            />
                            <QuickAccessCard 
                                title="Journal"
                                description="Track your trades and analyze performance"
                                path="/journal"
                            />
                            <QuickAccessCard 
                                title="Aftermarket Analyzer"
                                description="Analyze market data after market hours"
                                path="/aftermarket-analyzer"
                            />
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground rounded-lg p-6 shadow-sm">
            <h3 class="text-xl font-medium mb-2">{title}</h3>
            <p class="text-muted-foreground">{description}</p>
        </div>
    }
}

#[component]
fn QuickAccessCard(
    title: &'static str,
    description: &'static str,
    path: &'static str,
) -> impl IntoView {
    view! {
        <A
            href=path
            class="bg-card text-card-foreground rounded-lg p-6 shadow-sm hover:shadow transition-shadow"
        >
            <h3 class="text-xl font-medium mb-2">{title}</h3>
            <p class="text-muted-foreground mb-4">{description}</p>
            <div class="text-primary font-medium">Access now →</div>
        </A>
    }
}
