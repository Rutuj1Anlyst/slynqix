use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::components::analysis_table::*;
use crate::components::stat_card::*;
use crate::utils::api::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AnalysisResult {
    statistical: Vec<AnalysisRow>,
    visual: Vec<AnalysisRow>,
    indicator: Vec<AnalysisRow>,
    suggestions: Vec<String>,
}

#[component]
pub fn Console() -> impl IntoView {
    let (selected_symbol, set_selected_symbol) = create_signal("Nifty 50".to_string());
    let (analysis_result, set_analysis_result) = create_signal(None::<AnalysisResult>);
    let (is_loading, set_loading) = create_signal(false);
    let (current_time, set_current_time) = create_signal(String::new());
    
    // Update time every second
    create_effect(move |_| {
        let handle = window_set_interval_with_handle(
            move || {
                set_current_time.set(get_current_time());
            },
            1000,
        ).unwrap();
        
        on_cleanup(move || {
            window_clear_interval(handle);
        });
    });
    
    // Mock symbols
    let symbols = vec![
        "Nifty 50", "BankNifty", "FinNifty", "Sensex", "Midcap"
    ];
    
    let handle_symbol_change = move |ev: web_sys::Event| {
        let input = event_target_value(&ev);
        set_selected_symbol.set(input);
    };
    
    let analyze = move |_| {
        set_loading.set(true);
        
        // Mock API call
        set_timeout(
            move || {
                let mock_result = AnalysisResult {
                    statistical: vec![
                        AnalysisRow {
                            name: "Mean".to_string(),
                            value: "₹24,587.25".to_string(),
                            description: Some("Average price over the period".to_string()),
                        },
                        AnalysisRow {
                            name: "Standard Deviation".to_string(),
                            value: "₹235.42".to_string(),
                            description: Some("Volatility measure".to_string()),
                        },
                        AnalysisRow {
                            name: "Range".to_string(),
                            value: "₹980.75".to_string(),
                            description: Some("Difference between highest and lowest price".to_string()),
                        },
                    ],
                    visual: vec![
                        AnalysisRow {
                            name: "Trend".to_string(),
                            value: "Uptrend".to_string(),
                            description: Some("Current price direction".to_string()),
                        },
                        AnalysisRow {
                            name: "Pattern".to_string(),
                            value: "Double Bottom".to_string(),
                            description: Some("Recent price pattern".to_string()),
                        },
                        AnalysisRow {
                            name: "Support".to_string(),
                            value: "₹24,320.00".to_string(),
                            description: Some("Price level with buying interest".to_string()),
                        },
                        AnalysisRow {
                            name: "Resistance".to_string(),
                            value: "₹24,750.00".to_string(),
                            description: Some("Price level with selling interest".to_string()),
                        },
                    ],
                    indicator: vec![
                        AnalysisRow {
                            name: "RSI (14)".to_string(),
                            value: "58.32".to_string(),
                            description: Some("Neutral".to_string()),
                        },
                        AnalysisRow {
                            name: "MACD".to_string(),
                            value: "35.76".to_string(),
                            description: Some("Bullish".to_string()),
                        },
                        AnalysisRow {
                            name: "Moving Average (50)".to_string(),
                            value: "₹24,320.75".to_string(),
                            description: Some("Below current price".to_string()),
                        },
                        AnalysisRow {
                            name: "Moving Average (200)".to_string(),
                            value: "₹23,875.40".to_string(),
                            description: Some("Below current price".to_string()),
                        },
                    ],
                    suggestions: vec![
                        "Consider buying at ₹24,400 with stop loss at ₹24,320".to_string(),
                        "Sell target at ₹24,750".to_string(),
                        "Hold positions if already long".to_string(),
                    ],
                };
                
                set_analysis_result.set(Some(mock_result));
                set_loading.set(false);
            },
            1000,
        );
    };
    
    view! {
        <div>
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold">Console</h1>
                <div class="text-lg font-medium">{move || current_time.get()}</div>
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
                    <button 
                        class="px-4 py-2 bg-primary text-primary-foreground rounded-md"
                        on:click=analyze
                        disabled=is_loading
                    >
                        {move || if is_loading() { "Analyzing..." } else { "Analyze" }}
                    </button>
                </div>
            </div>
            
            {move || if let Some(result) = analysis_result.get() {
                view! {
                    <div class="space-y-6">
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <StatCard stat=StatData {
                                title: "Current Price".to_string(),
                                value: "₹24,587.25".to_string(),
                                description: Some("As of now".to_string()),
                            } />
                            <StatCard stat=StatData {
                                title: "Day Change".to_string(),
                                value: "+₹123.45 (+0.51%)".to_string(),
                                description: Some("Today's movement".to_string()),
                            } />
                            <StatCard stat=StatData {
                                title: "Volume".to_string(),
                                value: "128.5M".to_string(),
                                description: Some("Today's trading volume".to_string()),
                            } />
                        </div>
                        
                        <AnalysisTable 
                            title="Statistical Analysis"
                            rows=result.statistical
                        />
                        
                        <AnalysisTable 
                            title="Visual Analysis"
                            rows=result.visual
                        />
                        
                        <AnalysisTable 
                            title="Indicator Analysis"
                            rows=result.indicator
                        />
                        
                        <div class="bg-card text-card-foreground rounded-lg shadow-sm overflow-hidden">
                            <div class="p-4 border-b border-border">
                                <h3 class="text-lg font-medium">Suggestions</h3>
                            </div>
                            <div class="p-4">
                                <ul class="list-disc pl-5 space-y-2">
                                    {result.suggestions.into_iter().map(|suggestion| {
                                        view! { <li>{suggestion}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        </div>
                        
                        <div class="flex justify-end">
                            <A 
                                href="/aftermarket-analyzer" 
                                class="px-4 py-2 bg-secondary text-secondary-foreground rounded-md"
                            >
                                Go to Aftermarket Analyzer
                            </A>
                        </div>
                    </div>
                }
            } else {
                view! {
                    <div class="text-center py-10 text-muted-foreground">
                        Select a symbol and click "Analyze" to view analysis
                    </div>
                }
            }}
        </div>
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    let seconds = date.get_seconds();
    
    format!(
        "{:02}:{:02}:{:02}",
        hours, minutes, seconds
    )
}
