import React from 'react';
import type { Trade } from '../types/market';
import { cn } from '../lib/utils';
import { Clock, TrendingUp, TrendingDown } from 'lucide-react';

interface TradeTapeProps {
    trades: Trade[];
}

export const TradeTape: React.FC<TradeTapeProps> = ({ trades }) => {
    return (
        <div className="flex flex-col h-full bg-slate-900/50 backdrop-blur-md rounded-xl border border-slate-800 overflow-hidden font-mono text-xs">
            <div className="p-3 border-b border-slate-800 bg-slate-800/50 flex items-center gap-2">
                <Clock className="w-4 h-4 text-slate-400" />
                <h3 className="text-slate-200 font-bold uppercase tracking-wider">Recent Trades</h3>
            </div>

            <div className="flex-1 overflow-y-auto">
                <div className="grid grid-cols-3 px-3 py-2 text-[10px] uppercase text-slate-500 font-bold border-b border-slate-800/50 sticky top-0 bg-slate-900/90 z-20">
                    <div>Price</div>
                    <div className="text-center">Size</div>
                    <div className="text-right">Time</div>
                </div>

                <div className="flex flex-col">
                    {trades.length === 0 ? (
                        <div className="p-8 text-center text-slate-600 italic">
                            Waiting for trades...
                        </div>
                    ) : (
                        trades.map((trade, i) => (
                            <TradeRow key={`${trade.timestamp}-${i}`} trade={trade} />
                        ))
                    )}
                </div>
            </div>
        </div>
    );
};

const TradeRow: React.FC<{ trade: Trade }> = ({ trade }) => {
    const time = new Date(trade.timestamp).toLocaleTimeString([], {
        hour12: false,
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
    });

    return (
        <div className="grid grid-cols-3 px-3 py-2 border-b border-slate-800/30 hover:bg-slate-800/30 transition-colors animate-in fade-in slide-in-from-left-2 duration-300">
            <div className={cn(
                "font-bold flex items-center gap-1",
                trade.side === 'buy' ? "text-emerald-400" : "text-rose-400"
            )}>
                {trade.side === 'buy' ? (
                    <TrendingUp className="w-3 h-3 opacity-50" />
                ) : (
                    <TrendingDown className="w-3 h-3 opacity-50" />
                )}
                {parseFloat(trade.price).toFixed(2)}
            </div>
            <div className="text-slate-300 text-center">
                {parseFloat(trade.quantity).toFixed(4)}
            </div>
            <div className="text-slate-500 text-right text-[10px]">
                {time}
            </div>
        </div>
    );
};
