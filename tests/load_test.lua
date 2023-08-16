wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"external_url": "http://0.0.0.0:8080/fetch", "rest_method": "post", "request_headers": {}, "request_body": {"customer_params": [{"name": "Loan Number", "value": "5567873"}, {"name": "Customer Id", "value": "CUST8472"}]}, "request_ttl": 150, "response_call_back": "http:0.0.0.0:8080/"}'
wrk.headers["Host"] = "localhost"