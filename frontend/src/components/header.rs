use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(
    #[prop(into)] toggle_sidebar: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <header class="bg-background border-b border-border h-16 flex items-center justify-between px-4 sticky top-0 z-10">
            <div class="flex items-center">
                <button 
                    class="p-2 rounded-full hover:bg-secondary mr-2" 
                    on:click=toggle_sidebar
                >
                    // Menu Icon (you can replace with SVG)
                    <div class="w-5 h-5 flex flex-col justify-center gap-1">
                        <div class="w-full h-0.5 bg-foreground"></div>
                        <div class="w-full h-0.5 bg-foreground"></div>
                        <div class="w-full h-0.5 bg-foreground"></div>
                    </div>
                </button>
                
                <A href="/" class="text-xl font-bold">Slynqix</A>
            </div>
            
            <div class="flex space-x-4">
                <A href="/" class="p-2 rounded-full hover:bg-secondary">Home</A>
                <A href="/console" class="p-2 rounded-full hover:bg-secondary">Console</A>
                <A href="/mindsage" class="p-2 rounded-full hover:bg-secondary">Mindsage</A>
                <A href="/algo-trading" class="p-2 rounded-full hover:bg-secondary">Algo</A>
                <A href="/global-sentiment" class="p-2 rounded-full hover:bg-secondary">Sentiment</A>
                <A href="/journal" class="p-2 rounded-full hover:bg-secondary">Journal</A>
            </div>
            
            <div class="flex items-center space-x-4">
                <button class="p-2 rounded-full hover:bg-secondary">
                    // Notification Bell Icon
                    <div class="w-5 h-5">
                        // Bell icon placeholder - replace with SVG
                        <div class="w-4 h-4 border-2 border-foreground rounded-full relative">
                            <div class="absolute -bottom-1 left-1/2 transform -translate-x-1/2 w-1.5 h-1.5 bg-foreground"></div>
                        </div>
                    </div>
                </button>
                
                <A href="/profile" class="w-8 h-8 rounded-full bg-primary text-primary-foreground flex items-center justify-center">
                    // Profile placeholder
                    <span>U</span>
                </A>
            </div>
        </header>
    }
}
