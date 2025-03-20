use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OHLCVData {
    symbol: String,
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

#[component]
pub fn AftermarketAnalyzer() -> impl IntoView {
    let (selected_symbol, set_selected_symbol) = create_signal("Nifty 50".to_string());
    let (selected_date, set_selected_date) = create_signal("2025-03-20".to_string());
    let (ohlcv_data, set_ohlcv_data) = create_signal(Vec::<OHLCVData>::new());
    let (is_loading, set_loading) = create_signal(false);
    
    // Mock symbols
    let symbols = vec![
        "Nifty 50", "BankNifty", "FinNifty", "Sensex", "Midcap"
    ];
    
    let handle_symbol_change = move |ev: web_sys::Event| {
        let input = event_target_value(&ev);
        set_selected_symbol.set(input);
    };
    
    let handle_date_change = move |ev: web_sys::Event| {
        let input = event_target_value(&ev);
        set_selected_date.set(input);
    };
    
    let fetch_data = move |_| {
        set_loading.set(true);
        
        // Mock API call
        set_timeout(
            move || {
                let mock_data = vec![
                    OHLCVData {
                        symbol: selected_symbol.get(),
                        date: "2025-03-20".to_string(),
                        open: 24510.75,
                        high: 24650.30,
                        low: 24435.20,
                        close: 24587.25,
                        volume: 128500000,
                    },
                    OHLCVData {
                        symbol: selected_symbol.get(),
                        date: "2025-03-19".to_string(),
                        open: 24380.50,
                        high: 24530.80,
                        low: 24350.10,
                        close: 24463.80,
                        volume: 115300000,
                    },
                    OHLCVData {
                        symbol: selected_symbol.get(),
                        date: "2025-03-18".to_string(),
                        open: 24275.30,
                        high: 24410.60,
                        low: 24210.90,
                        close: 24385.40,
                        volume: 98700000,
                    },
                    OHLCVData {
                        symbol: selected_symbol.get(),
                        date: "2025-03-17".to_string(),
                        open: 24150.20,
                        high: 24320.40,
                        low: 24080.75,
                        close: 24275.30,
                        volume: 106400000,
                    },
                    OHLCVData {
                        symbol: selected_symbol.get(),
                        date: "2025-03-14".to_string(),
                        open: 24080.15,
                        high: 24220.35,
                        low: 23980.50,
                        close: 24150.20,
                        volume: 92800000,
                    },
                ];
                
                set_ohlcv_data.set(mock_data);
                set_loading.set(false);
            },
            1000,
        );
    };
    
    view! {
        <div>
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold">Aftermarket Analyzer</h1>
            </div>
            
            <div class="bg-card text-card-foreground rounded-lg p-6 shadow-sm mb-6">
                <div class="flex flex-col md:flex-row gap-4 items-end">
                    <div class="w-full md:w-64">
                        <label class="block text-sm font-medium mb-1">Select Symbol</label>
                        <select 
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            on:change=handle_symbol_change
                        >
                            {symbols.into_iter().map(|symbol| {
                                view! {
                                    <option 
                                        value={symbol} 
                                        selected={move || selected_symbol.get() == symbol}
                                    >
                                        {symbol}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>
                    <div class="w-full md:w-64">
                        <label class="block text-sm font-medium mb-1">Select Date</label>
                        <input 
                            type="date" 
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value=selected_date
                            on:change=handle_date_change
                        />
                    </div>
                    <button 
                        class="px-4 py-2 bg-primary text-primary-foreground rounded-md"
                        on:click=fetch_data
                        disabled=is_loading
                    >
                        {move || if is_loading() { "Fetching..." } else { "Fetch Data" }}
                    </button>
                </div>
            </div>
            
            {move || if !ohlcv_data.get().is_empty() {
                view! {
                    <div>
                        <div class="bg-card text-card-foreground rounded-lg shadow-sm overflow-hidden">
                            <div class="p-4 border-b border-border">
                                <h3 class="text-lg font-medium">OHLCV Data</h3>
                            </div>
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead>
                                        <tr class="border-b border-border">
                                            <th class="text-left p-3 text-muted-foreground font-medium">Date</th>
                                            <th class="text-left p-3 text-muted-foreground font-medium">Open</th>
                                            <th class="text-left p-3 text-muted-foreground font-medium">High</th>
                                            <th class="text-left p-3 text-muted-foreground font-medium">Low</th>
                                            <th class="text-left p-3 text-muted-foreground font-medium">Close</th>
                                            <th class="text-left p-3 text-muted-foreground font-medium">Volume</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {ohlcv_data.get().into_iter().map(|data| view! {
                                            <tr class="border-b border-border">
                                                <td class="p-3">{data.date}</td>
                                                <td class="p-3">{format!("₹{:.2}", data.open)}</td>
                                                <td class="p-3">{format!("₹{:.2}", data.high)}</td>
                                                <td class="p-3">{format!("₹{:.2}", data.low)}</td>
                                                <td class="p-3">{format!("₹{:.2}", data.close)}</td>
                                                <td class="p-3">{format!("{}", data.volume)}</td>
                                            </tr>
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
