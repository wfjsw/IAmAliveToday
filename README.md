# IAmAliveToday

Fix the CpDaily for you.

## Download

[Releases](releases) 

Note: Public releases would have Telemetry enabled for error-reporting. Build it yourself to disable this feature. 

## Config

Example:
```yaml
users: 
  - school: school_id_or_name
    username: 
    password: 
    address: 安徽省蚌埠市XX区XX路XX号
    device_info:
      model: Galaxy Nexus
      app_version: "9.0.12"
      system_version: "11.0.0"
      system_name: android
      device_id: <random 16 byte hex>
      lat: 36.123456 
      lon: 120.789012
      user_agent: Mozilla/5.0 (Linux; Android 8.0.0; MI 6 Build/OPR1.170623.027; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/92.0.4515.131 Mobile Safari/537.36 okhttp/3.12.4 cpdaily/9.0.12 wisedu/9.0.12
    actions:
      - type: CounselorFormFill
        force_submit: false
        form_data:
          - question: 所在地
            answer: 山东省/安徽市/蚌埠区/蚌埠住了大学
          - question: 体温
            answer: "37.2"
          - question: 身体状况
            answer: 健康
          - question: 近14天
            answer: 否
          - question: 你或你的共同居住人
            answer: 否
          - question: 健康码颜色
            answer: 绿色
          - question: 是否承诺
            answer: 是
  - school: 
    ...

```
