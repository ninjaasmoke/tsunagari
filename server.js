const http = require('http');
const axios = require('axios');
const uuid = require('uuid');

function getAckId() {
  const tsunaPrefix = 'TSUNA';
  const randomUuidPart = uuid.v4().substring(tsunaPrefix.length).replaceAll('-', '');
  const customUuid = tsunaPrefix + randomUuidPart;
  return customUuid;
}

async function makeExternalRequest(requestData) {
  try {
    const response = await axios({
      method: requestData.rest_method,
      url: requestData.external_url,
      headers: requestData.request_headers,
      data: requestData.request_body,
      timeout: Math.min(requestData.request_ttl || 165, 165) * 1000
    });
    return response.data;
  } catch (error) {
    throw error;
  }
}

async function makeCallbackRequest(callbackUrl, data) {
  try {
    const response = await axios({
      method: "POST",
      url: callbackUrl,
      data: data
    });
    return response.data;
  } catch (error) {
    throw error;
  }
}

const server = http.createServer(async (req, res) => {
  if (req.method === 'POST' && req.url === '/createRequest') {
    let data = '';

    req.on('data', chunk => {
      data += chunk;
    });

    req.on('end', async () => {
      try {
        const requestData = JSON.parse(data);
        const ackId = getAckId();
        const ackResponse = {
          status: 'success',
          statusCode: 201,
          message: 'Request received and ACK sent',
          ackId: ackId, // generate a random UUID here
          requestData: requestData
        };
        res.statusCode = 201;
        res.setHeader('Content-Type', 'application/json');
        res.end(JSON.stringify(ackResponse));
        
        let externalResponse;

        try {
          externalResponse = await makeExternalRequest(requestData);
        } catch (error) {
          externalResponse = {
            status: 'error',
            statusCode: error.response.status,
            error: error.response.data,
          }
        }
        await makeCallbackRequest(requestData.response_call_back, {
          response: externalResponse,
          ackId: ackId
        });
      } catch (error) {
        console.error('Error :', error);
        res.statusCode = 400;
        res.end('Bad Request');
      }
    });
  } else {
    res.statusCode = 404;
    res.end('Not Found');
  }
});

const PORT = 3000;

server.listen(PORT, () => {
  console.log(`Server is listening on port ${PORT}`);
});
