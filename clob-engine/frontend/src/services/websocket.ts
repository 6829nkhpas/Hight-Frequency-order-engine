import { MarketMessage } from '../types/market';

type MessageCallback = (msg: MarketMessage) => void;

class WebSocketService {
    private socket: WebSocket | null = null;
    private url: string;
    private callbacks: Set<MessageCallback> = new Set();
    private reconnectTimer: number | null = null;
    private maxReconnectAttempts = 10;
    private reconnectAttempts = 0;
    private reconnectInterval = 3000;

    constructor(url: string = 'ws://localhost:3000/ws/market') {
        this.url = url;
    }

    connect() {
        if (this.socket?.readyState === WebSocket.OPEN) return;

        console.log(`Connecting to WebSocket at ${this.url}...`);
        this.socket = new WebSocket(this.url);

        this.socket.onopen = () => {
            console.log('WebSocket connected');
            this.reconnectAttempts = 0;
            if (this.reconnectTimer) {
                window.clearTimeout(this.reconnectTimer);
                this.reconnectTimer = null;
            }
        };

        this.socket.onmessage = (event) => {
            try {
                const data: MarketMessage = JSON.parse(event.data);
                this.callbacks.forEach((cb) => cb(data));
            } catch (err) {
                console.error('Failed to parse WebSocket message:', err);
            }
        };

        this.socket.onclose = () => {
            console.log('WebSocket disconnected');
            this.scheduleReconnect();
        };

        this.socket.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.socket?.close();
        };
    }

    private scheduleReconnect() {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.error('Max reconnect attempts reached');
            return;
        }

        if (this.reconnectTimer) return;

        this.reconnectAttempts++;
        console.log(`Scheduling reconnect attempt ${this.reconnectAttempts} in ${this.reconnectInterval}ms...`);

        this.reconnectTimer = window.setTimeout(() => {
            this.reconnectTimer = null;
            this.connect();
        }, this.reconnectInterval);
    }

    subscribe(callback: MessageCallback) {
        this.callbacks.add(callback);
        return () => {
            this.callbacks.delete(callback);
        };
    }

    disconnect() {
        if (this.reconnectTimer) {
            window.clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
        this.socket?.close();
        this.socket = null;
    }
}

// Singleton instance
export const wsService = new WebSocketService();
