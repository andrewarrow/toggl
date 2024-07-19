curl 'https://track.toggl.com/api/v9/time_entries?meta=true' \ -H 'Content-Type: application/json' -H "Cookie: $TOGGLE_COOKIE" \
--data-raw '{
  "created_with": "Snowball",
  "pid": 123,
  "tid": 456,
  "start": "2024-07-18T16:00:00.000Z", 
  "stop": "2024-07-19T00:00:00.000Z", 
  "wid": 7001234,
  "duration": 28800,
  "description": "desc",
  "billable": false,
  "tags": [],
  "project_name": "App",
  "project_color": "#566614",
  "project_active": true, 
  "client_name": "name", "project_billable": false
}'
