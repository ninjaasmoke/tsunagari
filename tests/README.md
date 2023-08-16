# load_test.js

This file is a load test script that uses the k6 JavaScript library to make a POST request to the `http://localhost:3000/createRequest` endpoint. The request payload includes the following data:

* `external_url`: The URL of the external service that will be called.
* `rest_method`: The HTTP method to use for the request.
* `request_headers`: A map of HTTP headers to be sent with the request.
* `request_body`: The request body data.
* `request_ttl`: The maximum amount of time (in milliseconds) that the request is allowed to take before it is considered a failure.
* `response_call_back`: The URL of the endpoint that will be called to process the response from the external service.

The script first creates a JSON payload with the specified data. Then, it creates an HTTP header object with the `Content-Type` header set to `application/json`. Finally, it makes a POST request to the `http://localhost:3000/createRequest` endpoint, passing the payload and headers as arguments.

The script then logs the processing time for the request. If the response status code is not 201 (Created), the script logs an error message.

## Usage

To use this load test script, you will need to install the k6 library and the npm package dependencies. Then, you can run the script using the following command:

```k6 run load_test.js```

This will run the load test for 1 minute (the default duration) with 10 virtual users (the default number of VUs). You can change the duration and number of VUs by using the -d and -u command-line flags, respectively.

For example, to run the load test for 5 minutes with 50 VUs, you would use the following command:

```k6 run -u 10 -d 60s load_test.js```

## Output

The k6 library will output a variety of metrics to the console, including:

- The number of requests made.
- The total number of errors.
- The average processing time.
- The percentiles of the processing time.