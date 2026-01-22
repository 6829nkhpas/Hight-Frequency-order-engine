import { useEffect, useState, useCallback } from 'react';
import { wsService } from '../services/websocket';
import type { MarketMessage, OrderBookState, Trade } from '../types/market';

export function useMarketData() {
    const [orderBook, setOrderBook] = useState<OrderBookState>({
        bids: [],
        asks: [],
        bestBid: null,
        bestAsk: null,
        spread: null,
    });
    const [trades, setTrades] = useState<Trade[]>([]);
    const [isConnected, setIsConnected] = useState(false);

    const handleMessage = useCallback((msg: MarketMessage) => {
        switch (msg.type) {
            case 'connected':
                setIsConnected(true);
                break;
            case 'trade':
                setTrades((prev) => [msg, ...prev].slice(0, 50));
                break;
            case 'order_book':
                const bids = msg.bids.map(([price, quantity]) => ({ price, quantity }));
                const asks = msg.asks.map(([price, quantity]) => ({ price, quantity }));

                // Calculate spread
                let spread: string | null = null;
                if (msg.best_bid && msg.best_ask) {
                    spread = (parseFloat(msg.best_ask) - parseFloat(msg.best_bid)).toFixed(2);
                }

                setOrderBook({
                    bids,
                    asks,
                    bestBid: msg.best_bid,
                    bestAsk: msg.best_ask,
                    spread,
                });
                break;
        }
    }, []);

    useEffect(() => {
        wsService.connect();
        const unsubscribe = wsService.subscribe(handleMessage);

        return () => {
            unsubscribe();
        };
    }, [handleMessage]);

    return { orderBook, trades, isConnected };
}
