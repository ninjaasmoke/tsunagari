const http = require('http');
const axios = require('axios'); // Add this line to use Axios for asynchronous HTTP requests

const server = http.createServer(async (req, res) => {
  if (req.method === 'POST' && req.url === '/createRequest') {
    let data = '';

    req.on('data', chunk => {
      data += chunk;
    });

    req.on('end', async () => {
      try {
        const requestData = JSON.parse(data);
        const ackResponse = {
          status: 'success',
          statusCode: 201,
          message: 'Request received and ACK sent',
          requestData: requestData
        };
        res.statusCode = 201;
        res.setHeader('Content-Type', 'application/json');
        res.end(JSON.stringify(ackResponse));

        // Make asynchronous external request
        try {
          const externalResponse = await axios({
            method: requestData.rest_method,
            url: requestData.external_url,
            headers: requestData.request_headers,
            data: requestData.request_body,
            timeout: Math.min(requestData.request_ttl || 165, 165) * 1000
          })
          console.log('External request successful. Response:', externalResponse.data);
          try {
            const callback = await axios({
              method: "POST",
              url: requestData.response_call_back,
              data: externalResponse.data
            })
          } catch (error) {
            console.error("callback failed: ", error.message)
          }
        } catch (error) {
          console.error('Error making external request:', error.message);
        }
      } catch (error) {
        console.error('Error parsing JSON:', error);
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
