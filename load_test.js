import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
    const payload = JSON.stringify({
        external_url: "http://0.0.0.0:8080/fetch",
        rest_method: "POST",
        request_headers: {},
        request_body: {
            customer_params: [
                { name: "Loan Number", value: "5567873" },
                { name: "Customer Id", value: "CUST8472" }
            ]
        },
        request_ttl: 150,
        response_call_back: "http://0.0.0.0:8080/"
    });

    const headers = { 'Content-Type': 'application/json' };
    const url = 'http://localhost:3000/createRequest';

    const startTime = new Date();
    const response = http.post(url, payload, { headers: headers });
    const endTime = new Date();

    const processingTime = endTime - startTime;

    if (response.status !== 201) {
        console.error(`Request failed with status ${response.status}`);
    }
}
