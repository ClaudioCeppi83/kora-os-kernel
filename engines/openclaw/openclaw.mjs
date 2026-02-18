#!/usr/bin/env node

import * as readline from 'node:readline';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

console.log("OpenClaw Engine v0.1 (Mock) Initialized.");

rl.on('line', (line) => {
  const input = line.trim();
  if (!input) return;

  if (input.startsWith("SYSTEM")) {
      console.log(`System Acknowledged: ${input.substring(7)}`);
  } else if (input.startsWith("KNOWLEDGE")) {
      const query = input.substring(10);
      console.log(`Neuron Triggered: I have received your query about "${query}". Analysis pending.`);
  } else {
      console.error(`Unknown Protocol: ${input}`);
  }
});

// Keep process alive
setInterval(() => {}, 1000);

// Handle signals
process.on('SIGTERM', () => process.exit(0));
process.on('SIGINT', () => process.exit(0));
