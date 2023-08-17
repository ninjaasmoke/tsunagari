package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/google/uuid"
)

const MAX_TTL = 60

type RequestData struct {
	ExternalUrl      string `json:"external_url"`
	RequestBody      string `json:"request_body"`
	RestMethod       string `json:"rest_method"`
	RequestHeaders   string `json:"request_headers"`
	ResponseCallBack string `json:"response_call_back"`
	RequestTtl       int    `json:"request_ttl"`
	RefId            string `json:"ref_id"`
}

type AckResponse struct {
	Status      string      `json:"status"`
	StatusCode  int         `json:"status_code"`
	Message     string      `json:"message"`
	AckID       string      `json:"ack_id"`
	RequestData RequestData `json:"request_data"`
}

type CallBackRequest struct {
	RefId   string      `json:"refId"`
	Status  string      `json:"status"`
	Payload interface{} `json:"payload"`
}

func min[T int](a, b T) T {
	if a < b {
		return a
	}
	return b
}

func createRequestHandler(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/createRequest" {
		http.Error(w, "404 not found.", http.StatusNotFound)
		return
	}
	if r.Method != "POST" {
		http.Error(w, "Method is not supported.", http.StatusNotFound)
		return
	}

	var t RequestData
	err := json.NewDecoder(r.Body).Decode(&t)

	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	uuidWithPrefix := "TSUNA" + uuid.New().String()

	ackData := AckResponse{
		Status:      "success",
		StatusCode:  201,
		Message:     "Request Created",
		AckID:       uuidWithPrefix,
		RequestData: t,
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(ackData)

	req, err := http.NewRequest(t.RestMethod, t.ExternalUrl, bytes.NewBuffer([]byte(t.RequestBody)))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	var requestHeadersMap map[string]string
	err = json.Unmarshal([]byte(t.RequestHeaders), &requestHeadersMap)
	if err != nil {
		fmt.Println("Error decoding headers:", err)
		return
	}

	for key, value := range requestHeadersMap {
		req.Header.Add(key, value)
	}

	go func() {
		client := &http.Client{
			Timeout: time.Duration(min(t.RequestTtl, MAX_TTL)) * time.Second,
		}
		res, err := client.Do(req)

		if err != nil {
			fmt.Println("Error sending request:", err)

			callbackPayload := err.Error()
			callbackStatus := "error"

			callbackReqData := CallBackRequest{
				RefId:   t.RefId,
				Status:  callbackStatus,
				Payload: callbackPayload,
			}
			callbackReqJSON, err := json.Marshal(callbackReqData)
			if err != nil {
				fmt.Println("Error marshaling callback request:", err)
				return
			}
			callbackReq, err := http.NewRequest("POST", t.ResponseCallBack, bytes.NewBuffer(callbackReqJSON))
			if err != nil {
				fmt.Println("Error creating callback request:", err)
				return
			}
			callbackRes, err := client.Do(callbackReq)
			if err != nil {
				fmt.Println("Error sending callback request:", err)
				return
			}
			defer callbackRes.Body.Close()

			fmt.Println("Callback response status:", callbackRes.Status)
			return
		}

		defer res.Body.Close()

		var responseBody []byte
		responseBody, err = io.ReadAll(res.Body)
		if err != nil {
			fmt.Println("Error reading response:", err)
			return
		}

		callbackPayload := responseBody
		callbackStatus := "success"

		callbackReqData := CallBackRequest{
			RefId:   t.RefId,
			Status:  callbackStatus,
			Payload: callbackPayload,
		}
		callbackReqJSON, err := json.Marshal(callbackReqData)
		if err != nil {
			fmt.Println("Error marshaling callback request:", err)
			return
		}

		callbackReq, err := http.NewRequest("POST", t.ResponseCallBack, bytes.NewBuffer(callbackReqJSON))
		if err != nil {
			fmt.Println("Error creating callback request:", err)
			return
		}
		callbackRes, err := client.Do(callbackReq)
		if err != nil {
			fmt.Println("Error sending callback request:", err)
			return
		}
		defer callbackRes.Body.Close()
	}()
}

func main() {
	http.HandleFunc("/createRequest", createRequestHandler)

	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}
}
