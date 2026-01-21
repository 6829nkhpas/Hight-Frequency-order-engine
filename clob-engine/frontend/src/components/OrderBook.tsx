import React from 'react';
import { cn } from '../lib/utils';

interface OrderBookProps {
    bids: { price: string; quantity: string }[];
    asks: { price: string; quantity: string }[];
    spread: string | null;
}

export const OrderBook: React.FC<OrderBookProps> = ({ bids, asks, spread }) => {
    // Calculate max quantity for depth bars
    const maxQty = Math.max(
        ...bids.map((b) => parseFloat(b.quantity)),
        ...asks.map((a) => parseFloat(a.quantity)),
        1
    );

    return (
        <div className="flex flex-col h-full bg-slate-900/50 backdrop-blur-md rounded-xl border border-slate-800 overflow-hidden font-mono text-xs">
            <div className="p-3 border-b border-slate-800 bg-slate-800/50 flex justify-between items-center">
                <h3 className="text-slate-200 font-bold uppercase tracking-wider">Order Book</h3>
                {spread && (
                    <span className="text-slate-400">
                        Spread: <span className="text-slate-200 font-medium">{spread}</span>
                    </span>
                )}
            </div>

            <div className="flex flex-col flex-1 overflow-y-auto">
                {/* Asks (Sells) - Sorted High to Low (showing lowest at bottom) */}
                <div className="flex flex-col-reverse">
                    {asks.slice(0, 15).map((ask, i) => (
                        <PriceLevelRow
                            key={`ask-${i}`}
                            price={ask.price}
                            quantity={ask.quantity}
                            type="sell"
                            maxQty={maxQty}
                        />
                    ))}
                </div>

                {/* Spread / Mid Market Divider */}
                <div className="py-2 px-3 bg-slate-800/30 border-y border-slate-800/50 flex justify-center items-center">
                    <span className="text-slate-500 font-bold tracking-widest text-[10px] uppercase">
                        Market Spread
                    </span>
                </div>

                {/* Bids (Buys) - Sorted High to Low */}
                <div className="flex flex-col">
                    {bids.slice(0, 15).map((bid, i) => (
                        <PriceLevelRow
                            key={`bid-${i}`}
                            price={bid.price}
                            quantity={bid.quantity}
                            type="buy"
                            maxQty={maxQty}
                        />
                    ))}
                </div>
            </div>

            <div className="p-2 border-t border-slate-800 grid grid-cols-2 text-slate-500 text-[10px] uppercase font-bold text-center">
                <div>Price</div>
                <div>Size</div>
            </div>
        </div>
    );
};

interface PriceLevelRowProps {
    price: string;
    quantity: string;
    type: 'buy' | 'sell';
    maxQty: number;
}

const PriceLevelRow: React.FC<PriceLevelRowProps> = ({ price, quantity, type, maxQty }) => {
    const qtyNum = parseFloat(quantity);
    const depthPercent = (qtyNum / maxQty) * 100;

    return (
        <div className="relative group transition-colors hover:bg-slate-800/50 h-6 flex items-center px-3 cursor-pointer">
            {/* Depth Bar Background */}
            <div
                className={cn(
                    "absolute right-0 top-0 bottom-0 opacity-20 transition-all duration-300",
                    type === 'buy' ? "bg-emerald-500" : "bg-rose-500"
                )}
                style={{ width: `${depthPercent}%` }}
            />

            <div className="relative flex w-full justify-between items-center z-10">
                <span className={cn(
                    "font-bold transition-all group-hover:scale-105 origin-left",
                    type === 'buy' ? "text-emerald-400" : "text-rose-400"
                )}>
                    {parseFloat(price).toFixed(2)}
                </span>
                <span className="text-slate-300">
                    {parseFloat(quantity).toFixed(4)}
                </span>
            </div>
        </div>
    );
};
