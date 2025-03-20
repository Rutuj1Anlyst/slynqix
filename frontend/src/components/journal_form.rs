use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Option<String>,
    pub date: String,
    pub symbol: String,
    pub quantity: i32,
    pub buy_price: f64,
    pub sell_price: f64,
    pub action: String,
    pub fees: f64,
    pub pnl: f64,
    pub notes: String,
}

#[component]
pub fn JournalForm(
    #[prop(into)] on_save: Callback<JournalEntry>,
    #[prop(default)] selected_date: String,
) -> impl IntoView {
    let (entry, set_entry) = create_signal(JournalEntry {
        id: None,
        date: selected_date,
        symbol: String::new(),
        quantity: 0,
        buy_price: 0.0,
        sell_price: 0.0,
        action: "Buy".to_string(),
        fees: 0.0,
        pnl: 0.0,
        notes: String::new(),
    });
    
    // Update PnL when relevant fields change
    create_effect(move |_| {
        let current = entry.get();
        let pnl = (current.sell_price - current.buy_price) * current.quantity as f64 - current.fees;
        set_entry.update(|e| e.pnl = pnl);
    });
    
    let handle_input = move |field: &'static str, ev: web_sys::Event| {
        let target = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
        let value = target.value();
        
        set_entry.update(|entry| {
            match field {
                "symbol" => entry.symbol = value,
                "quantity" => entry.quantity = value.parse().unwrap_or(0),
                "buy_price" => entry.buy_price = value.parse().unwrap_or(0.0),
                "sell_price" => entry.sell_price = value.parse().unwrap_or(0.0),
                "action" => entry.action = value,
                "fees" => entry.fees = value.parse().unwrap_or(0.0),
                "notes" => entry.notes = value,
                _ => {}
            }
        });
    };
    
    let save = move |_| {
        on_save.call(entry.get());
        
        // Reset form
        set_entry.update(|e| {
            e.symbol = String::new();
            e.quantity = 0;
            e.buy_price = 0.0;
            e.sell_price = 0.0;
            e.action = "Buy".to_string();
            e.fees = 0.0;
            e.pnl = 0.0;
            e.notes = String::new();
        });
    };
    
    let cancel = move |_| {
        // Reset form
        set_entry.update(|e| {
            e.symbol = String::new();
            e.quantity = 0;
            e.buy_price = 0.0;
            e.sell_price = 0.0;
            e.action = "Buy".to_string();
            e.fees = 0.0;
            e.pnl = 0.0;
            e.notes = String::new();
        });
    };
    
    view! {
        <div class="space-y-6">
            <div class="bg-card text-card-foreground rounded-lg shadow-sm p-6">
                <h3 class="text-lg font-medium mb-4">Trade Details</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm font-medium mb-1">Symbol</label>
                        <input 
                            type="text" 
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value={move || entry.get().symbol}
                            on:input=move |ev| handle_input("symbol", ev)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">Quantity</label>
                        <input 
                            type="number" 
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value={move || entry.get().quantity.to_string()}
                            on:input=move |ev| handle_input("quantity", ev)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">Buy Price</label><input 
                            type="number" 
                            step="0.01"
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value={move || entry.get().buy_price.to_string()}
                            on:input=move |ev| handle_input("buy_price", ev)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">Sell Price</label>
                        <input 
                            type="number" 
                            step="0.01"
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value={move || entry.get().sell_price.to_string()}
                            on:input=move |ev| handle_input("sell_price", ev)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">Action</label>
                        <select 
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            on:change=move |ev| handle_input("action", ev)
                        >
                            <option selected={move || entry.get().action == "Buy"}>Buy</option>
                            <option selected={move || entry.get().action == "Sell"}>Sell</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">Fees</label>
                        <input 
                            type="number" 
                            step="0.01"
                            class="w-full px-3 py-2 border border-input rounded-md" 
                            value={move || entry.get().fees.to_string()}
                            on:input=move |ev| handle_input("fees", ev)
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium mb-1">PnL</label>
                        <input 
                            type="number" 
                            class="w-full px-3 py-2 border border-input rounded-md bg-muted" 
                            value={move || format!("{:.2}", entry.get().pnl)}
                            readonly=true
                        />
                    </div>
                </div>
                <div class="mt-4">
                    <label class="block text-sm font-medium mb-1">Notes</label>
                    <textarea 
                        class="w-full px-3 py-2 border border-input rounded-md" 
                        rows="3"
                        on:input=move |ev| handle_input("notes", ev)
                    >{move || entry.get().notes}</textarea>
                </div>
                <div class="mt-4 flex justify-end space-x-2">
                    <button 
                        class="px-4 py-2 bg-primary text-primary-foreground rounded-md"
                        on:click=save
                    >
                        Save
                    </button>
                    <button 
                        class="px-4 py-2 bg-secondary text-secondary-foreground rounded-md"
                        on:click=cancel
                    >
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn JournalTable(entries: Vec<JournalEntry>) -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground rounded-lg shadow-sm overflow-hidden mt-6">
            <div class="p-4 border-b border-border">
                <h3 class="text-lg font-medium">Trade History</h3>
            </div>
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-border">
                            <th class="text-left p-3 text-muted-foreground font-medium">Symbol</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Quantity</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Buy Price</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Sell Price</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Action</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Fees</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">PnL</th>
                            <th class="text-left p-3 text-muted-foreground font-medium">Notes</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries.into_iter().map(|entry| view! {
                            <tr class="border-b border-border">
                                <td class="p-3">{entry.symbol}</td>
                                <td class="p-3">{entry.quantity.to_string()}</td>
                                <td class="p-3">{"₹".to_string() + &entry.buy_price.to_string()}</td>
                                <td class="p-3">{"₹".to_string() + &entry.sell_price.to_string()}</td>
                                <td class="p-3">{entry.action}</td>
                                <td class="p-3">{"₹".to_string() + &entry.fees.to_string()}</td>
                                <td class="p-3">
                                    <span class={if entry.pnl >= 0.0 { "text-green-500" } else { "text-red-500" }}>
                                        {format!("₹{:.2}", entry.pnl)}
                                    </span>
                                </td>
                                <td class="p-3">{entry.notes}</td>
                            </tr>
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
