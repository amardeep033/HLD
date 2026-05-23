-> Protects from overwhelming/crashing of API server (from brute force attack or resource exhaustion)

-> Rate limiting : Number of req user/users can make per t per user/app : if more, block and resp err(429-too many request)

-> Throttling : slowing down rate of request, instead of outright rejecting them
   Static - Fixed max request rate
   Dynamic - based on CPU, RAM and queue length
   Adapting - based on AIML

-> Store request count/token in Redis 

-> For rate limiting : 
    Token Bucket Algorithm – allows short bursts within limits (most common).
    Fixed Window Counter  – counts requests within a fixed time window.
    Sliding Window Log / Sliding Window Counter – more accurate smoothing over time.

-> For throttling : 
    Leaky Bucket - Requests are queued and processed at a steady rate, ensuring smooth traffic flow and preventing sudden bursts.
