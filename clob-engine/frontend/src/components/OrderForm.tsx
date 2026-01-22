import React, { useState } from 'react';
import { Send, TrendingUp, TrendingDown, LayoutDashboard, Activity } from 'lucide-react';
import { cn } from '../lib/utils';

export const OrderForm: React.FC = () => {
    const [side, setSide] = useState<'buy' | 'sell'>('buy');
    const [price, setPrice] = useState('');
    const [quantity, setQuantity] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [status, setStatus] = useState<{ type: 'success' | 'error', msg: string } | null>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsSubmitting(true);
        setStatus(null);

        try {
            const response = await fetch('http://localhost:3000/api/orders', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    side,
                    price: parseFloat(price),
                    quantity: parseFloat(quantity),
                }),
            });

            const data = await response.json();

            if (response.ok) {
                setStatus({ type: 'success', msg: 'Order submitted!' });
                setPrice('');
                setQuantity('');
            } else {
                setStatus({ type: 'error', msg: data.message || 'Submission failed' });
            }
        } catch (err) {
            setStatus({ type: 'error', msg: 'Engine unreachable' });
        } finally {
            setIsSubmitting(false);
            setTimeout(() => setStatus(null), 3000);
        }
    };

    return (
        <div className="bg-slate-900/50 backdrop-blur-md rounded-xl border border-slate-800 overflow-hidden flex flex-col">
            <div className="p-3 border-b border-slate-800 bg-slate-800/50 flex items-center gap-2">
                <Send className="w-4 h-4 text-slate-400" />
                <h3 className="text-slate-200 font-bold uppercase tracking-wider">Place Order</h3>
            </div>

            <form onSubmit={handleSubmit} className="p-4 space-y-4">
                {/* Side Toggle */}
                <div className="grid grid-cols-2 gap-2 p-1 bg-slate-950 rounded-lg border border-slate-800">
                    <button
                        type="button"
                        onClick={() => setSide('buy')}
                        className={cn(
                            "py-2 rounded-md font-bold transition-all flex items-center justify-center gap-2",
                            side === 'buy'
                                ? "bg-emerald-500 text-white shadow-lg shadow-emerald-500/20"
                                : "text-slate-500 hover:text-slate-300"
                        )}
                    >
                        <TrendingUp className="w-4 h-4" />
                        BUY
                    </button>
                    <button
                        type="button"
                        onClick={() => setSide('sell')}
                        className={cn(
                            "py-2 rounded-md font-bold transition-all flex items-center justify-center gap-2",
                            side === 'sell'
                                ? "bg-rose-500 text-white shadow-lg shadow-rose-500/20"
                                : "text-slate-500 hover:text-slate-300"
                        )}
                    >
                        <TrendingDown className="w-4 h-4" />
                        SELL
                    </button>
                </div>

                {/* Inputs */}
                <div className="space-y-3">
                    <div>
                        <label className="block text-[10px] uppercase font-bold text-slate-500 mb-1 ml-1">Limit Price</label>
                        <input
                            type="number"
                            step="0.01"
                            required
                            value={price}
                            onChange={(e) => setPrice(e.target.value)}
                            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-100 focus:outline-none focus:ring-1 focus:ring-slate-600 transition-all font-mono"
                            placeholder="0.00"
                        />
                    </div>
                    <div>
                        <label className="block text-[10px] uppercase font-bold text-slate-500 mb-1 ml-1">Quantity</label>
                        <input
                            type="number"
                            step="0.0001"
                            required
                            value={quantity}
                            onChange={(e) => setQuantity(e.target.value)}
                            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-100 focus:outline-none focus:ring-1 focus:ring-slate-600 transition-all font-mono"
                            placeholder="0.0000"
                        />
                    </div>
                </div>

                <button
                    type="submit"
                    disabled={isSubmitting}
                    className={cn(
                        "w-full py-3 rounded-lg font-bold text-sm tracking-widest transition-all disabled:opacity-50 flex items-center justify-center gap-2",
                        side === 'buy'
                            ? "bg-emerald-600 hover:bg-emerald-500 text-white active:scale-[0.98]"
                            : "bg-rose-600 hover:bg-rose-500 text-white active:scale-[0.98]"
                    )}
                >
                    {isSubmitting ? "PROCESSING..." : `PLACE ${side.toUpperCase()} ORDER`}
                </button>

                {status && (
                    <div className={cn(
                        "p-3 rounded-lg text-xs font-bold text-center animate-in zoom-in duration-300",
                        status.type === 'success' ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
                    )}>
                        {status.msg}
                    </div>
                )}
            </form>

            <div className="mt-auto p-4 bg-slate-800/20 border-t border-slate-800">
                <div className="flex items-center gap-4">
                    <div className="flex items-center gap-1.5">
                        <LayoutDashboard className="w-3.5 h-3.5 text-slate-500" />
                        <span className="text-[10px] text-slate-400 font-bold uppercase">Trading BTC/USD</span>
                    </div>
                    <div className="flex items-center gap-1.5">
                        <Activity className="w-3.5 h-3.5 text-emerald-500 animate-pulse" />
                        <span className="text-[10px] text-slate-400 font-bold uppercase">Market Live</span>
                    </div>
                </div>
            </div>
        </div>
    );
};
