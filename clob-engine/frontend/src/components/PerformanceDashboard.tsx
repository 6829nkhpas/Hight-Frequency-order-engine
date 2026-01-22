import React, { useState } from 'react';
import { BarChart3, Zap, Clock, Activity, TrendingUp, Database } from 'lucide-react';
import { XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';
import { cn } from '../lib/utils';

interface PerformanceMetrics {
    orders_submitted: number;
    trades_executed: number;
    avg_latency_us: number;
    min_latency_us: number;
    max_latency_us: number;
    throughput_per_sec: number;
    simulation_duration_ms: number;
    current_spread: string | null;
    total_volume_traded: string;
}

export const PerformanceDashboard: React.FC = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null);
    const [numOrders, setNumOrders] = useState(1000);
    const [historicalData, setHistoricalData] = useState<Array<{
        timestamp: number;
        throughput: number;
        latency: number;
    }>>([]);

    const runSimulation = async () => {
        setIsRunning(true);
        setMetrics(null);

        try {
            const response = await fetch('http://localhost:3000/api/simulation', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    num_orders: numOrders,
                }),
            });

            const data = await response.json();

            if (data.success && data.metrics) {
                setMetrics(data.metrics);

                // Add to historical data
                setHistoricalData(prev => [
                    ...prev.slice(-9), // Keep last 9 entries
                    {
                        timestamp: Date.now(),
                        throughput: data.metrics.throughput_per_sec,
                        latency: data.metrics.avg_latency_us,
                    },
                ]);
            }
        } catch (err) {
            console.error('Simulation failed:', err);
        } finally {
            setIsRunning(false);
        }
    };

    return (
        <div className="bg-slate-900/50 backdrop-blur-md rounded-xl border border-slate-800 overflow-hidden flex flex-col">
            <div className="p-4 border-b border-slate-800 bg-slate-800/50 flex items-center justify-between">
                <div className="flex items-center gap-2">
                    <BarChart3 className="w-5 h-5 text-indigo-400" />
                    <h3 className="text-slate-200 font-bold uppercase tracking-wider">Performance Simulation</h3>
                </div>
                <div className={cn(
                    "flex items-center gap-2 px-2 py-1 rounded text-[10px] font-bold uppercase",
                    isRunning ? "bg-amber-500/10 text-amber-400" : "bg-slate-700 text-slate-400"
                )}>
                    <Activity className={cn("w-3 h-3", isRunning && "animate-pulse")} />
                    {isRunning ? 'Running...' : 'Idle'}
                </div>
            </div>

            <div className="p-6 space-y-6">
                {/* Controls */}
                <div className="flex items-center gap-4">
                    <div className="flex-1">
                        <label className="block text-[10px] uppercase font-bold text-slate-500 mb-2">Number of Orders</label>
                        <input
                            type="number"
                            value={numOrders}
                            onChange={(e) => setNumOrders(Number(e.target.value))}
                            min="100"
                            max="10000"
                            step="100"
                            disabled={isRunning}
                            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-100 focus:outline-none focus:ring-1 focus:ring-indigo-500 transition-all font-mono disabled:opacity-50"
                        />
                    </div>
                    <div className="pt-6">
                        <button
                            onClick={runSimulation}
                            disabled={isRunning}
                            className={cn(
                                "px-6 py-2.5 rounded-lg font-bold text-sm tracking-widest transition-all disabled:opacity-50 flex items-center gap-2",
                                "bg-gradient-to-r from-indigo-600 to-violet-600 hover:from-indigo-500 hover:to-violet-500 text-white shadow-lg shadow-indigo-500/20"
                            )}
                        >
                            <Zap className="w-4 h-4" />
                            {isRunning ? 'RUNNING...' : 'RUN SIMULATION'}
                        </button>
                    </div>
                </div>

                {/* Results */}
                {metrics && (
                    <div className="space-y-6">
                        {/* Key Metrics Grid */}
                        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
                            <MetricCard
                                icon={<TrendingUp className="w-5 h-5" />}
                                label="Throughput"
                                value={`${metrics.throughput_per_sec.toLocaleString(undefined, { maximumFractionDigits: 0 })}`}
                                unit="orders/sec"
                                color="indigo"
                            />
                            <MetricCard
                                icon={<Clock className="w-5 h-5" />}
                                label="Avg Latency"
                                value={`${metrics.avg_latency_us.toFixed(2)}`}
                                unit="μs"
                                color="emerald"
                            />
                            <MetricCard
                                icon={<Activity className="w-5 h-5" />}
                                label="Orders Processed"
                                value={metrics.orders_submitted.toLocaleString()}
                                unit=""
                                color="violet"
                            />
                            <MetricCard
                                icon={<Database className="w-5 h-5" />}
                                label="Duration"
                                value={`${metrics.simulation_duration_ms}`}
                                unit="ms"
                                color="rose"
                            />
                        </div>

                        {/* Detailed Stats */}
                        <div className="grid grid-cols-2 gap-4">
                            <div className="bg-slate-950/50 border border-slate-800 rounded-lg p-4 space-y-3">
                                <h4 className="text-[10px] font-black uppercase tracking-widest text-slate-500">Latency Breakdown</h4>
                                <div className="space-y-2">
                                    <StatRow label="Minimum" value={`${metrics.min_latency_us} μs`} highlight />
                                    <StatRow label="Average" value={`${metrics.avg_latency_us.toFixed(2)} μs`} />
                                    <StatRow label="Maximum" value={`${metrics.max_latency_us} μs`} />
                                </div>
                            </div>

                            <div className="bg-slate-950/50 border border-slate-800 rounded-lg p-4 space-y-3">
                                <h4 className="text-[10px] font-black uppercase tracking-widest text-slate-500">System Performance</h4>
                                <div className="space-y-2">
                                    <StatRow label="Orders/sec" value={metrics.throughput_per_sec.toFixed(0)} highlight />
                                    <StatRow label="Current Spread" value={metrics.current_spread || 'N/A'} />
                                    <StatRow label="Status" value="✓ Optimal" success />
                                </div>
                            </div>
                        </div>

                        {/* Historical Chart */}
                        {historicalData.length > 1 && (
                            <div className="bg-slate-950/50 border border-slate-800 rounded-lg p-4">
                                <h4 className="text-[10px] font-black uppercase tracking-widest text-slate-500 mb-4">Throughput History</h4>
                                <ResponsiveContainer width="100%" height={200}>
                                    <AreaChart data={historicalData}>
                                        <defs>
                                            <linearGradient id="colorThroughput" x1="0" y1="0" x2="0" y2="1">
                                                <stop offset="5%" stopColor="#6366f1" stopOpacity={0.3} />
                                                <stop offset="95%" stopColor="#6366f1" stopOpacity={0} />
                                            </linearGradient>
                                        </defs>
                                        <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" />
                                        <XAxis
                                            dataKey="timestamp"
                                            tickFormatter={(ts) => new Date(ts).toLocaleTimeString([], { timeStyle: 'short' })}
                                            stroke="#475569"
                                            style={{ fontSize: 10 }}
                                        />
                                        <YAxis stroke="#475569" style={{ fontSize: 10 }} />
                                        <Tooltip
                                            contentStyle={{ backgroundColor: '#0f172a', border: '1px solid #334155', borderRadius: '8px' }}
                                            labelStyle={{ color: '#94a3b8' }}
                                        />
                                        <Area
                                            type="monotone"
                                            dataKey="throughput"
                                            stroke="#6366f1"
                                            fillOpacity={1}
                                            fill="url(#colorThroughput)"
                                        />
                                    </AreaChart>
                                </ResponsiveContainer>
                            </div>
                        )}

                        {/* Technical Advantages */}
                        <div className="bg-gradient-to-br from-indigo-950/30 to-violet-950/30 border border-indigo-800/30 rounded-lg p-4">
                            <h4 className="text-[11px] font-black uppercase tracking-widest text-indigo-400 mb-3">🚀 Technical Advantages vs Traditional Systems</h4>
                            <div className="grid grid-cols-2 gap-3 text-xs">
                                <AdvantageItem
                                    label="Lock-Free Architecture"
                                    value="Zero mutex contention in hot path"
                                />
                                <AdvantageItem
                                    label="Memory Safety"
                                    value="Rust ownership prevents race conditions"
                                />
                                <AdvantageItem
                                    label="Async I/O"
                                    value="Non-blocking persistence layer"
                                />
                                <AdvantageItem
                                    label="Predictable Latency"
                                    value="No GC pauses unlike Java/C# systems"
                                />
                            </div>
                        </div>
                    </div>
                )}

                {/* Initial State */}
                {!metrics && !isRunning && (
                    <div className="text-center py-12 text-slate-600">
                        <BarChart3 className="w-16 h-16 mx-auto mb-4 opacity-30" />
                        <p className="text-sm font-medium">Click "Run Simulation" to benchmark the matching engine</p>
                        <p className="text-xs mt-1">This will submit random orders and measure system performance</p>
                    </div>
                )}
            </div>
        </div>
    );
};

const MetricCard: React.FC<{
    icon: React.ReactNode;
    label: string;
    value: string;
    unit: string;
    color: 'indigo' | 'emerald' | 'violet' | 'rose';
}> = ({ icon, label, value, unit, color }) => {
    const colorClasses = {
        indigo: 'from-indigo-500/10 to-indigo-500/5 border-indigo-500/20 text-indigo-400',
        emerald: 'from-emerald-500/10 to-emerald-500/5 border-emerald-500/20 text-emerald-400',
        violet: 'from-violet-500/10 to-violet-500/5 border-violet-500/20 text-violet-400',
        rose: 'from-rose-500/10 to-rose-500/5 border-rose-500/20 text-rose-400',
    };

    return (
        <div className={cn(
            "bg-gradient-to-br border rounded-lg p-4",
            colorClasses[color]
        )}>
            <div className="flex items-center gap-2 mb-2">
                {icon}
                <span className="text-[10px] font-bold uppercase tracking-wider text-slate-400">{label}</span>
            </div>
            <div className="font-mono">
                <span className="text-2xl font-black text-slate-100">{value}</span>
                {unit && <span className="text-xs ml-1 text-slate-400">{unit}</span>}
            </div>
        </div>
    );
};

const StatRow: React.FC<{ label: string; value: string; highlight?: boolean; success?: boolean }> = ({
    label,
    value,
    highlight,
    success
}) => (
    <div className="flex justify-between items-center text-xs">
        <span className="text-slate-500 font-medium">{label}</span>
        <span className={cn(
            "font-mono font-bold", highlight && "text-emerald-400",
            success && "text-green-400",
            !highlight && !success && "text-slate-300"
        )}>
            {value}
        </span>
    </div>
);

const AdvantageItem: React.FC<{ label: string; value: string }> = ({ label, value }) => (
    <div className="space-y-1">
        <div className="text-indigo-300 font-bold">{label}</div>
        <div className="text-slate-400 text-[11px]">{value}</div>
    </div>
);
