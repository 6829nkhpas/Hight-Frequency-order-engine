export type Side = 'buy' | 'sell';

export interface Trade {
    type: 'trade';
    price: string;
    quantity: string;
    side: Side;
    timestamp: number;
}

export interface OrderBookUpdate {
    type: 'order_book';
    best_bid: string | null;
    best_ask: string | null;
    bids: [string, string][]; // [price, quantity]
    asks: [string, string][];
}

export interface ConnectedMessage {
    type: 'connected';
    message: string;
}

export type MarketMessage = Trade | OrderBookUpdate | ConnectedMessage;

export interface OrderBookState {
    bids: { price: string; quantity: string }[];
    asks: { price: string; quantity: string }[];
    bestBid: string | null;
    bestAsk: string | null;
    spread: string | null;
}
