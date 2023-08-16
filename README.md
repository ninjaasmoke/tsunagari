# Reverse Proxy Asynchronous Node.js Server
## Architecture Overview:
The server is built using Node.js and utilizes the built-in http module for handling incoming HTTP requests. Additionally, it employs the Axios library to make asynchronous HTTP requests to external URLs.

## Server Design:
- HTTP Server Creation: The server is created using the http.createServer() function. It listens on a specified port (in this case, 3000) for incoming HTTP requests.

- Request Handling: When a request is received, the server's request handler function is executed. It inspects the method and URL of the incoming request to determine the appropriate action.

- /createRequest Endpoint: The server includes an endpoint /createRequest that only accepts HTTP POST requests. This endpoint is used for creating reverse proxy requests.

- Request Data Processing: Upon receiving a POST request to /createRequest, the server reads the JSON data from the request body. It then parses the JSON to extract the required information for the reverse proxy request.

- ACK Response: After successfully parsing the JSON data, the server immediately sends an ACK (acknowledgment) response back to the client. This response confirms the receipt of the request.

- Asynchronous External Request: After sending the ACK response, the server asynchronously makes a new HTTP request to the external URL specified in the JSON data. It uses the HTTP method (rest_method), request headers (request_headers), request body (request_body), and a timeout based on the minimum of request_ttl and 165 seconds.

- Response Handling: The server waits for the response from the external URL. If the external request is successful, the server logs the response data. If there's an error, it logs an error message.

## Functionality:
- The server listens on port 3000 for incoming HTTP requests.

- It provides an /createRequest endpoint that accepts POST requests.

- Upon receiving a POST request to /createRequest, the server immediately sends an ACK response back to the client.

- After sending the ACK response, the server asynchronously makes a new HTTP request to the external_url specified in the JSON data.

- The server includes the request method, headers, body, and a timeout in the external request.

- The server handles the external response, logging the response data upon success and error messages upon failure.

## Use Cases:
This server is designed to act as a reverse proxy, facilitating the creation of asynchronous reverse proxy requests to external URLs. It can be utilized in scenarios where you want to receive requests, acknowledge them promptly, and then make parallel requests to external services while handling responses asynchronously.

By following this architecture and design, you have created a simple yet effective asynchronous Node.js server capable of acting as a reverse proxy. This server enhances the responsiveness and efficiency of handling external requests while providing a seamless user experience.