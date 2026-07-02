const express = require('express');
const http = require('http');
const WebSocket = require('ws');
const path = require('path');
const { Kafka, Partitioners } = require('kafkajs');
const { createClient } = require('redis');
require('dotenv').config({ path: path.join(__dirname, '../.env') });

const app = express();
app.use(express.json());

const server = http.createServer(app);
const wss = new WebSocket.Server({ noServer: true });

const PORT = process.env.SERVER_PORT || 3000;
if (PORT == 8080) {
    console.error("DO NOT USE PORT 8080 as per requirements.");
    process.exit(1);
}

const REDPANDA_BROKERS = process.env.REDPANDA_BROKERS ? process.env.REDPANDA_BROKERS.split(',') : ['localhost:9092'];
const REDIS_URL = process.env.REDIS_URL || 'redis://localhost:6379';

// Kafka Setup
const kafka = new Kafka({
    clientId: 'logrider-server',
    brokers: REDPANDA_BROKERS,
    connectionTimeout: 3000,
    enforceRequestTimeout: false
});
const producer = kafka.producer({ createPartitioner: Partitioners.LegacyPartitioner });

// Redis Setup
const redisClient = createClient({ url: REDIS_URL });
redisClient.on('error', (err) => console.error('Redis Client Error', err));

let subscriber;

(async () => {
    try {
        await producer.connect();
        console.log('Connected to Redpanda producer');

        await redisClient.connect();
        console.log('Connected to Redis');

        subscriber = redisClient.duplicate();
        await subscriber.connect();
        
        await subscriber.subscribe('ws-logs', (message) => {
            wss.clients.forEach(client => {
                if (client.readyState === WebSocket.OPEN) {
                    client.send(message);
                }
            });
        });
        console.log('Subscribed to Redis channel ws-logs');

    } catch (e) {
        console.error('Initialization error:', e);
    }
})();

// WebSocket Upgrade handling
server.on('upgrade', (request, socket, head) => {
    if (request.url === '/api/ws') {
        wss.handleUpgrade(request, socket, head, (ws) => {
            wss.emit('connection', ws, request);
        });
    } else {
        socket.destroy();
    }
});

wss.on('connection', (ws) => {
    console.log('Client connected to WebSocket');
    ws.on('close', () => {
        console.log('Client disconnected from WebSocket');
    });
});

// Endpoints
app.get('/', (req, res) => {
    res.send('<h1>LogRider Server</h1><p>Running successfully.</p>');
});

app.get('/health', (req, res) => {
    res.status(200).json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.post('/login', (req, res) => {
    res.json({ token: 'sample-jwt-token' });
});

app.get('/dashboard', (req, res) => {
    res.sendFile(path.join(__dirname, 'dashboard.html'));
});

app.post('/api/logs', async (req, res) => {
    try {
        const logData = req.body;
        await producer.send({
            topic: 'logs-raw',
            messages: [
                { value: JSON.stringify(logData) },
            ],
        });
        res.status(202).json({ status: 'accepted' });
    } catch (error) {
        console.error('Error sending log to Redpanda:', error);
        res.status(500).json({ error: 'Internal Server Error' });
    }
});

server.listen(PORT, () => {
    console.log(`Server listening on port ${PORT}`);
});
