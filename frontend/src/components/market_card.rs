use leptos::*;

#[derive(Clone)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[component]
pub fn MarketCard(market: MarketData) -> impl IntoView {
    let is_positive = market.change >= 0.0;
    
    view! {
        <div class="bg-card text-card-foreground rounded-lg p-4 shadow-sm">
            <h3 class="text-lg font-medium">{market.symbol}</h3>
            <div class="mt-2">
                <p class="text-2xl font-bold">{format!("₹{:.2}", market.price)}</p>
                <div class=move || format!(
                    "flex items-center mt-1 {}",
                    if is_positive { "text-green-500" } else { "text-red-500" }
                )>
                    <span>
                        {move || format!("{}{:.2} ({}{:.2}%)", 
                            if is_positive { "+" } else { "" },
                            market.change,
                            if is_positive { "+" } else { "" },
                            market.change_percent
                        )}
                    </span>
                </div>
            </div>
        </div>
    }
}
