'use strict';
const { parentPort, workerData } = require('node:worker_threads');
const sdk = require(workerData.engineModule || '@7mlabs/astrology');

// One persistent worker processes native synchronous calculations sequentially.
parentPort.on('message', ({ id, requestJson, service = 'calculateJson' }) => {
  try {
    if (!['calculateJson', 'calculateFromLocalJson', 'resolveBirthTimeJson', 'renderChartSvgJson', 'capabilitiesJson'].includes(service)) throw new Error('Unknown SDK service');
    let raw;
    if (service === 'capabilitiesJson') raw = JSON.stringify({ data: sdk.getCapabilities(), errors: [], warnings: [] });
    else if (service === 'renderChartSvgJson') {
      raw = sdk.renderChartSvgRequestJson(requestJson);
    } else raw = sdk[service](requestJson);
    parentPort.postMessage({ id, result: JSON.parse(raw) });
  } catch (error) {
    parentPort.postMessage({ id, error: { name: error.name, message: error.message } });
  }
});
