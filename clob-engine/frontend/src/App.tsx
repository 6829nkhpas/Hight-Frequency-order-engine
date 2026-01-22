import { useMarketData } from './hooks/useMarketData';
import { OrderBook } from './components/OrderBook';
import { TradeTape } from './components/TradeTape';
import { OrderForm } from './components/OrderForm';
import { Activity, ShieldCheck, Zap } from 'lucide-react';
import { cn } from './lib/utils';

function App() {
  const { orderBook, trades, isConnected } = useMarketData();

  return (
    <div className="min-h-screen bg-[#020617] text-slate-200 font-sans selection:bg-indigo-500/30">
      {/* Header */}
      <header className="border-b border-slate-800/80 bg-slate-950/50 backdrop-blur-xl sticky top-0 z-50">
        <div className="max-w-[1600px] mx-auto px-6 h-16 flex items-center justify-between">
          <div className="flex items-center gap-8">
            <div className="flex items-center gap-2.5">
              <div className="w-9 h-9 bg-gradient-to-br from-indigo-500 to-violet-600 rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
                <Zap className="w-5 h-5 text-white" />
              </div>
              <h1 className="text-xl font-black tracking-tight text-white uppercase italic">
                C<span className="text-indigo-500">L</span>OB<span className="ml-1 text-[10px] not-italic font-bold text-slate-500 bg-slate-800 px-1.5 py-0.5 rounded uppercase tracking-tighter">Engine</span>
              </h1>
            </div>

            <nav className="hidden md:flex items-center gap-6 text-[11px] font-bold uppercase tracking-widest text-slate-500">
              <a href="#" className="text-white border-b-2 border-indigo-500 pb-0.5">Trade</a>
              <a href="#" className="hover:text-slate-300 transition-colors">History</a>
              <a href="#" className="hover:text-slate-300 transition-colors">Analytics</a>
            </nav>
          </div>

          <div className="flex items-center gap-4">
            <div className={cn(
              "flex items-center gap-2 px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase transition-all duration-500",
              isConnected
                ? "bg-emerald-500/5 text-emerald-400 border-emerald-500/20"
                : "bg-rose-500/5 text-rose-400 border-rose-500/20"
            )}>
              <Activity className={cn("w-3.5 h-3.5", isConnected && "animate-pulse")} />
              {isConnected ? 'Market Protocol Online' : 'Connecting to Node...'}
            </div>
            <div className="h-4 w-[1px] bg-slate-800 mx-1" />
            <div className="flex items-center gap-2 text-slate-500 text-[10px] font-bold uppercase">
              <ShieldCheck className="w-4 h-4" />
              <span className="hidden sm:inline">Encrypted Terminal</span>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-[1600px] mx-auto p-6 md:p-8">
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 h-[calc(100vh-160px)] min-h-[700px]">
          {/* Order Placement Panel */}
          <div className="lg:col-span-3 flex flex-col gap-6">
            <OrderForm />

            {/* Market Info Card */}
            <div className="flex-1 bg-slate-900/40 rounded-xl border border-slate-800/50 p-6 flex flex-col justify-between group hover:border-indigo-500/30 transition-all duration-500">
              <div>
                <h4 className="text-[10px] font-black uppercase tracking-widest text-indigo-400 mb-6">Market Statistics</h4>
                <div className="space-y-6">
                  <StatItem label="Symbol" value="BTC / USD" />
                  <StatItem label="Price" value={orderBook.bestAsk ? `$ ${parseFloat(orderBook.bestAsk).toLocaleString()}` : '---'} highlight />
                  <StatItem label="24h Volume" value="512.42 BTC" />
                  <StatItem label="Protocol" value="TCP/Rust v0.1" />
                </div>
              </div>
              <div className="pt-6 border-t border-slate-800/50">
                <p className="text-[10px] text-slate-500 font-medium leading-relaxed">
                  The matching engine operates on a Price-Time priority model. Orders are processed with sub-100μs latency.
                </p>
              </div>
            </div>
          </div>

          {/* Central Order Book */}
          <div className="lg:col-span-6 flex flex-col">
            <OrderBook
              bids={orderBook.bids}
              asks={orderBook.asks}
              spread={orderBook.spread}
            />
          </div>

          {/* Trade Tape and Charts */}
          <div className="lg:col-span-3 flex flex-col gap-6">
            <div className="flex-1">
              <TradeTape trades={trades} />
            </div>
          </div>
        </div>
      </main>

      {/* Footer / Status Bar */}
      <footer className="fixed bottom-0 left-0 right-0 border-t border-slate-800/50 bg-slate-950/80 backdrop-blur-md px-6 py-2">
        <div className="max-w-[1600px] mx-auto flex justify-between items-center text-[9px] font-black uppercase tracking-[0.2em] text-slate-500">
          <div className="flex items-center gap-6">
            <span className="flex items-center gap-2">
              <span className="w-1 h-1 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.8)]" />
              Matching Engine: Native
            </span>
            <span className="flex items-center gap-2">
              <span className="w-1 h-1 rounded-full bg-indigo-500" />
              Persistence: PostgreSQL
            </span>
          </div>
          <div className="flex items-center gap-4">
            <span>Latency: &lt; 100μs</span>
            <span>Uptime: 99.98%</span>
          </div>
        </div>
      </footer>
    </div>
  );
}

const StatItem = ({ label, value, highlight = false }: { label: string, value: string, highlight?: boolean }) => (
  <div className="flex justify-between items-baseline group/item">
    <span className="text-[10px] font-bold text-slate-500 uppercase tracking-wider group-hover/item:text-slate-400 transition-colors">{label}</span>
    <span className={cn(
      "font-mono font-bold tracking-tight transition-all duration-300",
      highlight ? "text-lg text-emerald-400 drop-shadow-[0_0_8px_rgba(52,211,153,0.3)]" : "text-slate-200"
    )}>
      {value}
    </span>
  </div>
);

export default App;
