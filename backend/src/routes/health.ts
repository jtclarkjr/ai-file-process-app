import { Hono } from 'hono';
import type { HealthResponse } from '../types';

const health = new Hono();

health.get('/health', (c) => {
  const response: HealthResponse = { status: 'OK' };
  return c.json(response);
});

export { health };
