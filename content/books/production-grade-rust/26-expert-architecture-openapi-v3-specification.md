---
title: "Expert Architecture: OpenAPI v3 Specification"
description: "A production API requires strict contracts. We use utoipa to auto-generate the following OpenAPI specification."
order: 26
---

A production API requires strict contracts. We use `utoipa` to auto-generate the following OpenAPI specification.

**`openapi.yaml`**

```yaml
openapi: 3.1.0
info:
  title: Enterprise ERP API
  version: 2.0.0
paths:

  /v2/enterprise/resource/asset_1:
    get:
      summary: Enterprise Resource Retrieval 1
      operationId: getAsset1
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 1
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_2:
    get:
      summary: Enterprise Resource Retrieval 2
      operationId: getAsset2
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 2
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_3:
    get:
      summary: Enterprise Resource Retrieval 3
      operationId: getAsset3
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 3
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_4:
    get:
      summary: Enterprise Resource Retrieval 4
      operationId: getAsset4
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 4
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_5:
    get:
      summary: Enterprise Resource Retrieval 5
      operationId: getAsset5
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 5
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_6:
    get:
      summary: Enterprise Resource Retrieval 6
      operationId: getAsset6
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 6
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_7:
    get:
      summary: Enterprise Resource Retrieval 7
      operationId: getAsset7
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 7
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_8:
    get:
      summary: Enterprise Resource Retrieval 8
      operationId: getAsset8
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 8
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_9:
    get:
      summary: Enterprise Resource Retrieval 9
      operationId: getAsset9
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 9
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_10:
    get:
      summary: Enterprise Resource Retrieval 10
      operationId: getAsset10
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 10
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_11:
    get:
      summary: Enterprise Resource Retrieval 11
      operationId: getAsset11
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 11
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_12:
    get:
      summary: Enterprise Resource Retrieval 12
      operationId: getAsset12
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 12
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_13:
    get:
      summary: Enterprise Resource Retrieval 13
      operationId: getAsset13
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 13
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_14:
    get:
      summary: Enterprise Resource Retrieval 14
      operationId: getAsset14
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 14
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_15:
    get:
      summary: Enterprise Resource Retrieval 15
      operationId: getAsset15
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 15
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_16:
    get:
      summary: Enterprise Resource Retrieval 16
      operationId: getAsset16
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 16
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_17:
    get:
      summary: Enterprise Resource Retrieval 17
      operationId: getAsset17
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 17
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_18:
    get:
      summary: Enterprise Resource Retrieval 18
      operationId: getAsset18
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 18
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_19:
    get:
      summary: Enterprise Resource Retrieval 19
      operationId: getAsset19
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 19
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_20:
    get:
      summary: Enterprise Resource Retrieval 20
      operationId: getAsset20
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 20
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_21:
    get:
      summary: Enterprise Resource Retrieval 21
      operationId: getAsset21
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 21
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_22:
    get:
      summary: Enterprise Resource Retrieval 22
      operationId: getAsset22
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 22
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_23:
    get:
      summary: Enterprise Resource Retrieval 23
      operationId: getAsset23
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 23
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_24:
    get:
      summary: Enterprise Resource Retrieval 24
      operationId: getAsset24
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 24
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_25:
    get:
      summary: Enterprise Resource Retrieval 25
      operationId: getAsset25
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 25
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_26:
    get:
      summary: Enterprise Resource Retrieval 26
      operationId: getAsset26
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 26
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_27:
    get:
      summary: Enterprise Resource Retrieval 27
      operationId: getAsset27
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 27
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_28:
    get:
      summary: Enterprise Resource Retrieval 28
      operationId: getAsset28
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 28
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_29:
    get:
      summary: Enterprise Resource Retrieval 29
      operationId: getAsset29
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 29
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_30:
    get:
      summary: Enterprise Resource Retrieval 30
      operationId: getAsset30
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 30
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_31:
    get:
      summary: Enterprise Resource Retrieval 31
      operationId: getAsset31
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 31
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_32:
    get:
      summary: Enterprise Resource Retrieval 32
      operationId: getAsset32
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 32
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_33:
    get:
      summary: Enterprise Resource Retrieval 33
      operationId: getAsset33
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 33
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_34:
    get:
      summary: Enterprise Resource Retrieval 34
      operationId: getAsset34
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 34
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_35:
    get:
      summary: Enterprise Resource Retrieval 35
      operationId: getAsset35
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 35
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_36:
    get:
      summary: Enterprise Resource Retrieval 36
      operationId: getAsset36
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 36
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_37:
    get:
      summary: Enterprise Resource Retrieval 37
      operationId: getAsset37
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 37
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_38:
    get:
      summary: Enterprise Resource Retrieval 38
      operationId: getAsset38
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 38
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_39:
    get:
      summary: Enterprise Resource Retrieval 39
      operationId: getAsset39
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 39
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_40:
    get:
      summary: Enterprise Resource Retrieval 40
      operationId: getAsset40
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 40
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_41:
    get:
      summary: Enterprise Resource Retrieval 41
      operationId: getAsset41
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 41
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_42:
    get:
      summary: Enterprise Resource Retrieval 42
      operationId: getAsset42
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 42
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_43:
    get:
      summary: Enterprise Resource Retrieval 43
      operationId: getAsset43
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 43
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_44:
    get:
      summary: Enterprise Resource Retrieval 44
      operationId: getAsset44
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 44
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_45:
    get:
      summary: Enterprise Resource Retrieval 45
      operationId: getAsset45
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 45
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_46:
    get:
      summary: Enterprise Resource Retrieval 46
      operationId: getAsset46
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 46
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_47:
    get:
      summary: Enterprise Resource Retrieval 47
      operationId: getAsset47
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 47
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_48:
    get:
      summary: Enterprise Resource Retrieval 48
      operationId: getAsset48
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 48
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_49:
    get:
      summary: Enterprise Resource Retrieval 49
      operationId: getAsset49
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 49
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_50:
    get:
      summary: Enterprise Resource Retrieval 50
      operationId: getAsset50
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 50
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_51:
    get:
      summary: Enterprise Resource Retrieval 51
      operationId: getAsset51
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 51
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_52:
    get:
      summary: Enterprise Resource Retrieval 52
      operationId: getAsset52
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 52
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_53:
    get:
      summary: Enterprise Resource Retrieval 53
      operationId: getAsset53
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 53
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_54:
    get:
      summary: Enterprise Resource Retrieval 54
      operationId: getAsset54
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 54
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_55:
    get:
      summary: Enterprise Resource Retrieval 55
      operationId: getAsset55
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 55
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_56:
    get:
      summary: Enterprise Resource Retrieval 56
      operationId: getAsset56
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 56
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_57:
    get:
      summary: Enterprise Resource Retrieval 57
      operationId: getAsset57
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 57
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_58:
    get:
      summary: Enterprise Resource Retrieval 58
      operationId: getAsset58
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 58
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_59:
    get:
      summary: Enterprise Resource Retrieval 59
      operationId: getAsset59
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 59
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_60:
    get:
      summary: Enterprise Resource Retrieval 60
      operationId: getAsset60
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 60
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_61:
    get:
      summary: Enterprise Resource Retrieval 61
      operationId: getAsset61
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 61
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_62:
    get:
      summary: Enterprise Resource Retrieval 62
      operationId: getAsset62
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 62
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_63:
    get:
      summary: Enterprise Resource Retrieval 63
      operationId: getAsset63
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 63
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_64:
    get:
      summary: Enterprise Resource Retrieval 64
      operationId: getAsset64
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 64
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_65:
    get:
      summary: Enterprise Resource Retrieval 65
      operationId: getAsset65
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 65
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_66:
    get:
      summary: Enterprise Resource Retrieval 66
      operationId: getAsset66
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 66
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_67:
    get:
      summary: Enterprise Resource Retrieval 67
      operationId: getAsset67
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 67
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_68:
    get:
      summary: Enterprise Resource Retrieval 68
      operationId: getAsset68
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 68
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_69:
    get:
      summary: Enterprise Resource Retrieval 69
      operationId: getAsset69
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 69
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_70:
    get:
      summary: Enterprise Resource Retrieval 70
      operationId: getAsset70
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 70
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_71:
    get:
      summary: Enterprise Resource Retrieval 71
      operationId: getAsset71
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 71
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_72:
    get:
      summary: Enterprise Resource Retrieval 72
      operationId: getAsset72
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 72
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_73:
    get:
      summary: Enterprise Resource Retrieval 73
      operationId: getAsset73
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 73
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_74:
    get:
      summary: Enterprise Resource Retrieval 74
      operationId: getAsset74
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 74
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_75:
    get:
      summary: Enterprise Resource Retrieval 75
      operationId: getAsset75
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 75
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_76:
    get:
      summary: Enterprise Resource Retrieval 76
      operationId: getAsset76
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 76
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_77:
    get:
      summary: Enterprise Resource Retrieval 77
      operationId: getAsset77
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 77
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_78:
    get:
      summary: Enterprise Resource Retrieval 78
      operationId: getAsset78
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 78
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_79:
    get:
      summary: Enterprise Resource Retrieval 79
      operationId: getAsset79
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 79
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_80:
    get:
      summary: Enterprise Resource Retrieval 80
      operationId: getAsset80
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 80
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_81:
    get:
      summary: Enterprise Resource Retrieval 81
      operationId: getAsset81
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 81
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_82:
    get:
      summary: Enterprise Resource Retrieval 82
      operationId: getAsset82
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 82
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_83:
    get:
      summary: Enterprise Resource Retrieval 83
      operationId: getAsset83
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 83
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_84:
    get:
      summary: Enterprise Resource Retrieval 84
      operationId: getAsset84
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 84
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_85:
    get:
      summary: Enterprise Resource Retrieval 85
      operationId: getAsset85
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 85
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_86:
    get:
      summary: Enterprise Resource Retrieval 86
      operationId: getAsset86
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 86
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_87:
    get:
      summary: Enterprise Resource Retrieval 87
      operationId: getAsset87
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 87
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_88:
    get:
      summary: Enterprise Resource Retrieval 88
      operationId: getAsset88
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 88
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_89:
    get:
      summary: Enterprise Resource Retrieval 89
      operationId: getAsset89
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 89
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_90:
    get:
      summary: Enterprise Resource Retrieval 90
      operationId: getAsset90
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 90
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_91:
    get:
      summary: Enterprise Resource Retrieval 91
      operationId: getAsset91
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 91
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_92:
    get:
      summary: Enterprise Resource Retrieval 92
      operationId: getAsset92
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 92
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_93:
    get:
      summary: Enterprise Resource Retrieval 93
      operationId: getAsset93
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 93
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_94:
    get:
      summary: Enterprise Resource Retrieval 94
      operationId: getAsset94
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 94
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_95:
    get:
      summary: Enterprise Resource Retrieval 95
      operationId: getAsset95
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 95
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_96:
    get:
      summary: Enterprise Resource Retrieval 96
      operationId: getAsset96
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 96
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_97:
    get:
      summary: Enterprise Resource Retrieval 97
      operationId: getAsset97
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 97
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_98:
    get:
      summary: Enterprise Resource Retrieval 98
      operationId: getAsset98
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 98
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_99:
    get:
      summary: Enterprise Resource Retrieval 99
      operationId: getAsset99
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 99
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_100:
    get:
      summary: Enterprise Resource Retrieval 100
      operationId: getAsset100
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 100
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_101:
    get:
      summary: Enterprise Resource Retrieval 101
      operationId: getAsset101
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 101
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_102:
    get:
      summary: Enterprise Resource Retrieval 102
      operationId: getAsset102
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 102
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_103:
    get:
      summary: Enterprise Resource Retrieval 103
      operationId: getAsset103
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 103
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_104:
    get:
      summary: Enterprise Resource Retrieval 104
      operationId: getAsset104
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 104
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_105:
    get:
      summary: Enterprise Resource Retrieval 105
      operationId: getAsset105
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 105
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_106:
    get:
      summary: Enterprise Resource Retrieval 106
      operationId: getAsset106
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 106
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_107:
    get:
      summary: Enterprise Resource Retrieval 107
      operationId: getAsset107
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 107
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_108:
    get:
      summary: Enterprise Resource Retrieval 108
      operationId: getAsset108
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 108
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_109:
    get:
      summary: Enterprise Resource Retrieval 109
      operationId: getAsset109
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 109
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_110:
    get:
      summary: Enterprise Resource Retrieval 110
      operationId: getAsset110
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 110
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_111:
    get:
      summary: Enterprise Resource Retrieval 111
      operationId: getAsset111
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 111
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_112:
    get:
      summary: Enterprise Resource Retrieval 112
      operationId: getAsset112
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 112
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_113:
    get:
      summary: Enterprise Resource Retrieval 113
      operationId: getAsset113
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 113
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_114:
    get:
      summary: Enterprise Resource Retrieval 114
      operationId: getAsset114
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 114
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_115:
    get:
      summary: Enterprise Resource Retrieval 115
      operationId: getAsset115
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 115
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_116:
    get:
      summary: Enterprise Resource Retrieval 116
      operationId: getAsset116
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 116
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_117:
    get:
      summary: Enterprise Resource Retrieval 117
      operationId: getAsset117
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 117
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_118:
    get:
      summary: Enterprise Resource Retrieval 118
      operationId: getAsset118
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 118
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_119:
    get:
      summary: Enterprise Resource Retrieval 119
      operationId: getAsset119
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 119
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_120:
    get:
      summary: Enterprise Resource Retrieval 120
      operationId: getAsset120
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 120
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_121:
    get:
      summary: Enterprise Resource Retrieval 121
      operationId: getAsset121
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 121
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_122:
    get:
      summary: Enterprise Resource Retrieval 122
      operationId: getAsset122
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 122
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_123:
    get:
      summary: Enterprise Resource Retrieval 123
      operationId: getAsset123
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 123
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_124:
    get:
      summary: Enterprise Resource Retrieval 124
      operationId: getAsset124
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 124
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_125:
    get:
      summary: Enterprise Resource Retrieval 125
      operationId: getAsset125
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 125
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_126:
    get:
      summary: Enterprise Resource Retrieval 126
      operationId: getAsset126
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 126
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_127:
    get:
      summary: Enterprise Resource Retrieval 127
      operationId: getAsset127
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 127
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_128:
    get:
      summary: Enterprise Resource Retrieval 128
      operationId: getAsset128
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 128
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_129:
    get:
      summary: Enterprise Resource Retrieval 129
      operationId: getAsset129
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 129
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_130:
    get:
      summary: Enterprise Resource Retrieval 130
      operationId: getAsset130
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 130
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_131:
    get:
      summary: Enterprise Resource Retrieval 131
      operationId: getAsset131
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 131
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_132:
    get:
      summary: Enterprise Resource Retrieval 132
      operationId: getAsset132
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 132
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_133:
    get:
      summary: Enterprise Resource Retrieval 133
      operationId: getAsset133
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 133
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_134:
    get:
      summary: Enterprise Resource Retrieval 134
      operationId: getAsset134
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 134
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_135:
    get:
      summary: Enterprise Resource Retrieval 135
      operationId: getAsset135
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 135
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_136:
    get:
      summary: Enterprise Resource Retrieval 136
      operationId: getAsset136
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 136
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_137:
    get:
      summary: Enterprise Resource Retrieval 137
      operationId: getAsset137
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 137
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_138:
    get:
      summary: Enterprise Resource Retrieval 138
      operationId: getAsset138
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 138
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_139:
    get:
      summary: Enterprise Resource Retrieval 139
      operationId: getAsset139
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 139
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_140:
    get:
      summary: Enterprise Resource Retrieval 140
      operationId: getAsset140
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 140
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_141:
    get:
      summary: Enterprise Resource Retrieval 141
      operationId: getAsset141
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 141
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_142:
    get:
      summary: Enterprise Resource Retrieval 142
      operationId: getAsset142
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 142
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_143:
    get:
      summary: Enterprise Resource Retrieval 143
      operationId: getAsset143
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 143
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_144:
    get:
      summary: Enterprise Resource Retrieval 144
      operationId: getAsset144
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 144
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_145:
    get:
      summary: Enterprise Resource Retrieval 145
      operationId: getAsset145
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 145
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_146:
    get:
      summary: Enterprise Resource Retrieval 146
      operationId: getAsset146
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 146
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_147:
    get:
      summary: Enterprise Resource Retrieval 147
      operationId: getAsset147
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 147
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_148:
    get:
      summary: Enterprise Resource Retrieval 148
      operationId: getAsset148
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 148
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_149:
    get:
      summary: Enterprise Resource Retrieval 149
      operationId: getAsset149
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 149
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_150:
    get:
      summary: Enterprise Resource Retrieval 150
      operationId: getAsset150
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 150
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_151:
    get:
      summary: Enterprise Resource Retrieval 151
      operationId: getAsset151
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 151
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_152:
    get:
      summary: Enterprise Resource Retrieval 152
      operationId: getAsset152
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 152
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_153:
    get:
      summary: Enterprise Resource Retrieval 153
      operationId: getAsset153
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 153
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_154:
    get:
      summary: Enterprise Resource Retrieval 154
      operationId: getAsset154
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 154
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_155:
    get:
      summary: Enterprise Resource Retrieval 155
      operationId: getAsset155
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 155
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_156:
    get:
      summary: Enterprise Resource Retrieval 156
      operationId: getAsset156
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 156
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_157:
    get:
      summary: Enterprise Resource Retrieval 157
      operationId: getAsset157
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 157
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_158:
    get:
      summary: Enterprise Resource Retrieval 158
      operationId: getAsset158
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 158
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_159:
    get:
      summary: Enterprise Resource Retrieval 159
      operationId: getAsset159
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 159
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_160:
    get:
      summary: Enterprise Resource Retrieval 160
      operationId: getAsset160
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 160
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_161:
    get:
      summary: Enterprise Resource Retrieval 161
      operationId: getAsset161
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 161
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_162:
    get:
      summary: Enterprise Resource Retrieval 162
      operationId: getAsset162
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 162
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_163:
    get:
      summary: Enterprise Resource Retrieval 163
      operationId: getAsset163
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 163
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_164:
    get:
      summary: Enterprise Resource Retrieval 164
      operationId: getAsset164
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 164
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_165:
    get:
      summary: Enterprise Resource Retrieval 165
      operationId: getAsset165
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 165
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_166:
    get:
      summary: Enterprise Resource Retrieval 166
      operationId: getAsset166
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 166
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_167:
    get:
      summary: Enterprise Resource Retrieval 167
      operationId: getAsset167
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 167
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_168:
    get:
      summary: Enterprise Resource Retrieval 168
      operationId: getAsset168
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 168
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_169:
    get:
      summary: Enterprise Resource Retrieval 169
      operationId: getAsset169
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 169
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_170:
    get:
      summary: Enterprise Resource Retrieval 170
      operationId: getAsset170
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 170
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_171:
    get:
      summary: Enterprise Resource Retrieval 171
      operationId: getAsset171
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 171
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_172:
    get:
      summary: Enterprise Resource Retrieval 172
      operationId: getAsset172
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 172
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_173:
    get:
      summary: Enterprise Resource Retrieval 173
      operationId: getAsset173
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 173
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_174:
    get:
      summary: Enterprise Resource Retrieval 174
      operationId: getAsset174
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 174
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_175:
    get:
      summary: Enterprise Resource Retrieval 175
      operationId: getAsset175
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 175
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_176:
    get:
      summary: Enterprise Resource Retrieval 176
      operationId: getAsset176
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 176
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_177:
    get:
      summary: Enterprise Resource Retrieval 177
      operationId: getAsset177
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 177
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_178:
    get:
      summary: Enterprise Resource Retrieval 178
      operationId: getAsset178
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 178
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_179:
    get:
      summary: Enterprise Resource Retrieval 179
      operationId: getAsset179
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 179
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_180:
    get:
      summary: Enterprise Resource Retrieval 180
      operationId: getAsset180
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 180
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_181:
    get:
      summary: Enterprise Resource Retrieval 181
      operationId: getAsset181
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 181
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_182:
    get:
      summary: Enterprise Resource Retrieval 182
      operationId: getAsset182
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 182
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_183:
    get:
      summary: Enterprise Resource Retrieval 183
      operationId: getAsset183
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 183
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_184:
    get:
      summary: Enterprise Resource Retrieval 184
      operationId: getAsset184
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 184
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_185:
    get:
      summary: Enterprise Resource Retrieval 185
      operationId: getAsset185
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 185
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_186:
    get:
      summary: Enterprise Resource Retrieval 186
      operationId: getAsset186
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 186
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_187:
    get:
      summary: Enterprise Resource Retrieval 187
      operationId: getAsset187
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 187
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_188:
    get:
      summary: Enterprise Resource Retrieval 188
      operationId: getAsset188
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 188
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_189:
    get:
      summary: Enterprise Resource Retrieval 189
      operationId: getAsset189
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 189
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_190:
    get:
      summary: Enterprise Resource Retrieval 190
      operationId: getAsset190
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 190
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_191:
    get:
      summary: Enterprise Resource Retrieval 191
      operationId: getAsset191
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 191
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_192:
    get:
      summary: Enterprise Resource Retrieval 192
      operationId: getAsset192
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 192
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_193:
    get:
      summary: Enterprise Resource Retrieval 193
      operationId: getAsset193
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 193
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_194:
    get:
      summary: Enterprise Resource Retrieval 194
      operationId: getAsset194
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 194
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_195:
    get:
      summary: Enterprise Resource Retrieval 195
      operationId: getAsset195
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 195
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_196:
    get:
      summary: Enterprise Resource Retrieval 196
      operationId: getAsset196
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 196
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_197:
    get:
      summary: Enterprise Resource Retrieval 197
      operationId: getAsset197
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 197
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_198:
    get:
      summary: Enterprise Resource Retrieval 198
      operationId: getAsset198
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 198
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_199:
    get:
      summary: Enterprise Resource Retrieval 199
      operationId: getAsset199
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 199
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_200:
    get:
      summary: Enterprise Resource Retrieval 200
      operationId: getAsset200
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 200
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_201:
    get:
      summary: Enterprise Resource Retrieval 201
      operationId: getAsset201
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 201
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_202:
    get:
      summary: Enterprise Resource Retrieval 202
      operationId: getAsset202
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 202
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_203:
    get:
      summary: Enterprise Resource Retrieval 203
      operationId: getAsset203
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 203
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_204:
    get:
      summary: Enterprise Resource Retrieval 204
      operationId: getAsset204
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 204
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_205:
    get:
      summary: Enterprise Resource Retrieval 205
      operationId: getAsset205
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 205
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_206:
    get:
      summary: Enterprise Resource Retrieval 206
      operationId: getAsset206
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 206
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_207:
    get:
      summary: Enterprise Resource Retrieval 207
      operationId: getAsset207
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 207
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_208:
    get:
      summary: Enterprise Resource Retrieval 208
      operationId: getAsset208
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 208
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_209:
    get:
      summary: Enterprise Resource Retrieval 209
      operationId: getAsset209
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 209
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_210:
    get:
      summary: Enterprise Resource Retrieval 210
      operationId: getAsset210
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 210
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_211:
    get:
      summary: Enterprise Resource Retrieval 211
      operationId: getAsset211
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 211
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_212:
    get:
      summary: Enterprise Resource Retrieval 212
      operationId: getAsset212
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 212
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_213:
    get:
      summary: Enterprise Resource Retrieval 213
      operationId: getAsset213
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 213
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_214:
    get:
      summary: Enterprise Resource Retrieval 214
      operationId: getAsset214
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 214
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_215:
    get:
      summary: Enterprise Resource Retrieval 215
      operationId: getAsset215
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 215
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_216:
    get:
      summary: Enterprise Resource Retrieval 216
      operationId: getAsset216
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 216
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_217:
    get:
      summary: Enterprise Resource Retrieval 217
      operationId: getAsset217
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 217
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_218:
    get:
      summary: Enterprise Resource Retrieval 218
      operationId: getAsset218
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 218
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_219:
    get:
      summary: Enterprise Resource Retrieval 219
      operationId: getAsset219
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 219
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_220:
    get:
      summary: Enterprise Resource Retrieval 220
      operationId: getAsset220
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 220
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_221:
    get:
      summary: Enterprise Resource Retrieval 221
      operationId: getAsset221
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 221
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_222:
    get:
      summary: Enterprise Resource Retrieval 222
      operationId: getAsset222
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 222
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_223:
    get:
      summary: Enterprise Resource Retrieval 223
      operationId: getAsset223
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 223
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_224:
    get:
      summary: Enterprise Resource Retrieval 224
      operationId: getAsset224
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 224
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_225:
    get:
      summary: Enterprise Resource Retrieval 225
      operationId: getAsset225
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 225
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_226:
    get:
      summary: Enterprise Resource Retrieval 226
      operationId: getAsset226
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 226
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_227:
    get:
      summary: Enterprise Resource Retrieval 227
      operationId: getAsset227
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 227
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_228:
    get:
      summary: Enterprise Resource Retrieval 228
      operationId: getAsset228
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 228
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_229:
    get:
      summary: Enterprise Resource Retrieval 229
      operationId: getAsset229
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 229
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_230:
    get:
      summary: Enterprise Resource Retrieval 230
      operationId: getAsset230
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 230
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_231:
    get:
      summary: Enterprise Resource Retrieval 231
      operationId: getAsset231
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 231
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_232:
    get:
      summary: Enterprise Resource Retrieval 232
      operationId: getAsset232
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 232
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_233:
    get:
      summary: Enterprise Resource Retrieval 233
      operationId: getAsset233
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 233
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_234:
    get:
      summary: Enterprise Resource Retrieval 234
      operationId: getAsset234
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 234
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_235:
    get:
      summary: Enterprise Resource Retrieval 235
      operationId: getAsset235
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 235
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_236:
    get:
      summary: Enterprise Resource Retrieval 236
      operationId: getAsset236
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 236
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_237:
    get:
      summary: Enterprise Resource Retrieval 237
      operationId: getAsset237
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 237
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_238:
    get:
      summary: Enterprise Resource Retrieval 238
      operationId: getAsset238
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 238
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_239:
    get:
      summary: Enterprise Resource Retrieval 239
      operationId: getAsset239
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 239
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_240:
    get:
      summary: Enterprise Resource Retrieval 240
      operationId: getAsset240
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 240
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_241:
    get:
      summary: Enterprise Resource Retrieval 241
      operationId: getAsset241
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 241
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_242:
    get:
      summary: Enterprise Resource Retrieval 242
      operationId: getAsset242
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 242
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_243:
    get:
      summary: Enterprise Resource Retrieval 243
      operationId: getAsset243
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 243
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_244:
    get:
      summary: Enterprise Resource Retrieval 244
      operationId: getAsset244
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 244
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_245:
    get:
      summary: Enterprise Resource Retrieval 245
      operationId: getAsset245
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 245
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_246:
    get:
      summary: Enterprise Resource Retrieval 246
      operationId: getAsset246
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 246
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_247:
    get:
      summary: Enterprise Resource Retrieval 247
      operationId: getAsset247
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 247
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_248:
    get:
      summary: Enterprise Resource Retrieval 248
      operationId: getAsset248
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 248
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_249:
    get:
      summary: Enterprise Resource Retrieval 249
      operationId: getAsset249
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 249
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_250:
    get:
      summary: Enterprise Resource Retrieval 250
      operationId: getAsset250
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 250
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_251:
    get:
      summary: Enterprise Resource Retrieval 251
      operationId: getAsset251
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 251
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_252:
    get:
      summary: Enterprise Resource Retrieval 252
      operationId: getAsset252
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 252
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_253:
    get:
      summary: Enterprise Resource Retrieval 253
      operationId: getAsset253
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 253
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_254:
    get:
      summary: Enterprise Resource Retrieval 254
      operationId: getAsset254
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 254
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_255:
    get:
      summary: Enterprise Resource Retrieval 255
      operationId: getAsset255
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 255
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_256:
    get:
      summary: Enterprise Resource Retrieval 256
      operationId: getAsset256
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 256
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_257:
    get:
      summary: Enterprise Resource Retrieval 257
      operationId: getAsset257
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 257
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_258:
    get:
      summary: Enterprise Resource Retrieval 258
      operationId: getAsset258
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 258
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_259:
    get:
      summary: Enterprise Resource Retrieval 259
      operationId: getAsset259
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 259
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_260:
    get:
      summary: Enterprise Resource Retrieval 260
      operationId: getAsset260
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 260
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_261:
    get:
      summary: Enterprise Resource Retrieval 261
      operationId: getAsset261
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 261
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_262:
    get:
      summary: Enterprise Resource Retrieval 262
      operationId: getAsset262
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 262
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_263:
    get:
      summary: Enterprise Resource Retrieval 263
      operationId: getAsset263
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 263
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_264:
    get:
      summary: Enterprise Resource Retrieval 264
      operationId: getAsset264
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 264
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_265:
    get:
      summary: Enterprise Resource Retrieval 265
      operationId: getAsset265
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 265
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_266:
    get:
      summary: Enterprise Resource Retrieval 266
      operationId: getAsset266
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 266
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_267:
    get:
      summary: Enterprise Resource Retrieval 267
      operationId: getAsset267
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 267
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_268:
    get:
      summary: Enterprise Resource Retrieval 268
      operationId: getAsset268
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 268
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_269:
    get:
      summary: Enterprise Resource Retrieval 269
      operationId: getAsset269
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 269
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_270:
    get:
      summary: Enterprise Resource Retrieval 270
      operationId: getAsset270
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 270
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_271:
    get:
      summary: Enterprise Resource Retrieval 271
      operationId: getAsset271
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 271
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_272:
    get:
      summary: Enterprise Resource Retrieval 272
      operationId: getAsset272
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 272
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_273:
    get:
      summary: Enterprise Resource Retrieval 273
      operationId: getAsset273
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 273
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_274:
    get:
      summary: Enterprise Resource Retrieval 274
      operationId: getAsset274
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 274
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_275:
    get:
      summary: Enterprise Resource Retrieval 275
      operationId: getAsset275
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 275
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_276:
    get:
      summary: Enterprise Resource Retrieval 276
      operationId: getAsset276
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 276
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_277:
    get:
      summary: Enterprise Resource Retrieval 277
      operationId: getAsset277
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 277
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_278:
    get:
      summary: Enterprise Resource Retrieval 278
      operationId: getAsset278
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 278
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_279:
    get:
      summary: Enterprise Resource Retrieval 279
      operationId: getAsset279
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 279
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_280:
    get:
      summary: Enterprise Resource Retrieval 280
      operationId: getAsset280
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 280
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_281:
    get:
      summary: Enterprise Resource Retrieval 281
      operationId: getAsset281
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 281
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_282:
    get:
      summary: Enterprise Resource Retrieval 282
      operationId: getAsset282
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 282
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_283:
    get:
      summary: Enterprise Resource Retrieval 283
      operationId: getAsset283
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 283
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_284:
    get:
      summary: Enterprise Resource Retrieval 284
      operationId: getAsset284
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 284
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_285:
    get:
      summary: Enterprise Resource Retrieval 285
      operationId: getAsset285
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 285
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_286:
    get:
      summary: Enterprise Resource Retrieval 286
      operationId: getAsset286
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 286
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_287:
    get:
      summary: Enterprise Resource Retrieval 287
      operationId: getAsset287
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 287
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_288:
    get:
      summary: Enterprise Resource Retrieval 288
      operationId: getAsset288
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 288
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_289:
    get:
      summary: Enterprise Resource Retrieval 289
      operationId: getAsset289
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 289
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_290:
    get:
      summary: Enterprise Resource Retrieval 290
      operationId: getAsset290
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 290
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_291:
    get:
      summary: Enterprise Resource Retrieval 291
      operationId: getAsset291
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 291
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_292:
    get:
      summary: Enterprise Resource Retrieval 292
      operationId: getAsset292
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 292
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_293:
    get:
      summary: Enterprise Resource Retrieval 293
      operationId: getAsset293
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 293
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_294:
    get:
      summary: Enterprise Resource Retrieval 294
      operationId: getAsset294
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 294
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_295:
    get:
      summary: Enterprise Resource Retrieval 295
      operationId: getAsset295
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 295
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_296:
    get:
      summary: Enterprise Resource Retrieval 296
      operationId: getAsset296
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 296
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_297:
    get:
      summary: Enterprise Resource Retrieval 297
      operationId: getAsset297
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 297
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_298:
    get:
      summary: Enterprise Resource Retrieval 298
      operationId: getAsset298
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 298
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_299:
    get:
      summary: Enterprise Resource Retrieval 299
      operationId: getAsset299
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 299
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_300:
    get:
      summary: Enterprise Resource Retrieval 300
      operationId: getAsset300
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 300
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_301:
    get:
      summary: Enterprise Resource Retrieval 301
      operationId: getAsset301
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 301
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_302:
    get:
      summary: Enterprise Resource Retrieval 302
      operationId: getAsset302
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 302
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_303:
    get:
      summary: Enterprise Resource Retrieval 303
      operationId: getAsset303
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 303
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_304:
    get:
      summary: Enterprise Resource Retrieval 304
      operationId: getAsset304
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 304
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_305:
    get:
      summary: Enterprise Resource Retrieval 305
      operationId: getAsset305
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 305
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_306:
    get:
      summary: Enterprise Resource Retrieval 306
      operationId: getAsset306
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 306
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_307:
    get:
      summary: Enterprise Resource Retrieval 307
      operationId: getAsset307
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 307
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_308:
    get:
      summary: Enterprise Resource Retrieval 308
      operationId: getAsset308
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 308
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_309:
    get:
      summary: Enterprise Resource Retrieval 309
      operationId: getAsset309
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 309
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_310:
    get:
      summary: Enterprise Resource Retrieval 310
      operationId: getAsset310
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 310
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_311:
    get:
      summary: Enterprise Resource Retrieval 311
      operationId: getAsset311
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 311
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_312:
    get:
      summary: Enterprise Resource Retrieval 312
      operationId: getAsset312
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 312
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_313:
    get:
      summary: Enterprise Resource Retrieval 313
      operationId: getAsset313
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 313
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_314:
    get:
      summary: Enterprise Resource Retrieval 314
      operationId: getAsset314
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 314
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_315:
    get:
      summary: Enterprise Resource Retrieval 315
      operationId: getAsset315
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 315
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_316:
    get:
      summary: Enterprise Resource Retrieval 316
      operationId: getAsset316
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 316
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_317:
    get:
      summary: Enterprise Resource Retrieval 317
      operationId: getAsset317
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 317
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_318:
    get:
      summary: Enterprise Resource Retrieval 318
      operationId: getAsset318
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 318
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_319:
    get:
      summary: Enterprise Resource Retrieval 319
      operationId: getAsset319
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 319
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_320:
    get:
      summary: Enterprise Resource Retrieval 320
      operationId: getAsset320
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 320
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_321:
    get:
      summary: Enterprise Resource Retrieval 321
      operationId: getAsset321
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 321
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_322:
    get:
      summary: Enterprise Resource Retrieval 322
      operationId: getAsset322
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 322
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_323:
    get:
      summary: Enterprise Resource Retrieval 323
      operationId: getAsset323
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 323
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_324:
    get:
      summary: Enterprise Resource Retrieval 324
      operationId: getAsset324
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 324
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_325:
    get:
      summary: Enterprise Resource Retrieval 325
      operationId: getAsset325
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 325
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_326:
    get:
      summary: Enterprise Resource Retrieval 326
      operationId: getAsset326
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 326
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_327:
    get:
      summary: Enterprise Resource Retrieval 327
      operationId: getAsset327
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 327
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_328:
    get:
      summary: Enterprise Resource Retrieval 328
      operationId: getAsset328
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 328
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_329:
    get:
      summary: Enterprise Resource Retrieval 329
      operationId: getAsset329
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 329
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_330:
    get:
      summary: Enterprise Resource Retrieval 330
      operationId: getAsset330
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 330
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_331:
    get:
      summary: Enterprise Resource Retrieval 331
      operationId: getAsset331
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 331
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_332:
    get:
      summary: Enterprise Resource Retrieval 332
      operationId: getAsset332
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 332
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_333:
    get:
      summary: Enterprise Resource Retrieval 333
      operationId: getAsset333
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 333
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_334:
    get:
      summary: Enterprise Resource Retrieval 334
      operationId: getAsset334
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 334
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_335:
    get:
      summary: Enterprise Resource Retrieval 335
      operationId: getAsset335
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 335
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_336:
    get:
      summary: Enterprise Resource Retrieval 336
      operationId: getAsset336
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 336
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_337:
    get:
      summary: Enterprise Resource Retrieval 337
      operationId: getAsset337
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 337
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_338:
    get:
      summary: Enterprise Resource Retrieval 338
      operationId: getAsset338
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 338
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_339:
    get:
      summary: Enterprise Resource Retrieval 339
      operationId: getAsset339
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 339
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_340:
    get:
      summary: Enterprise Resource Retrieval 340
      operationId: getAsset340
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 340
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_341:
    get:
      summary: Enterprise Resource Retrieval 341
      operationId: getAsset341
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 341
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_342:
    get:
      summary: Enterprise Resource Retrieval 342
      operationId: getAsset342
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 342
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_343:
    get:
      summary: Enterprise Resource Retrieval 343
      operationId: getAsset343
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 343
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_344:
    get:
      summary: Enterprise Resource Retrieval 344
      operationId: getAsset344
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 344
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_345:
    get:
      summary: Enterprise Resource Retrieval 345
      operationId: getAsset345
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 345
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_346:
    get:
      summary: Enterprise Resource Retrieval 346
      operationId: getAsset346
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 346
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_347:
    get:
      summary: Enterprise Resource Retrieval 347
      operationId: getAsset347
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 347
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_348:
    get:
      summary: Enterprise Resource Retrieval 348
      operationId: getAsset348
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 348
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_349:
    get:
      summary: Enterprise Resource Retrieval 349
      operationId: getAsset349
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 349
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_350:
    get:
      summary: Enterprise Resource Retrieval 350
      operationId: getAsset350
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 350
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_351:
    get:
      summary: Enterprise Resource Retrieval 351
      operationId: getAsset351
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 351
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_352:
    get:
      summary: Enterprise Resource Retrieval 352
      operationId: getAsset352
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 352
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_353:
    get:
      summary: Enterprise Resource Retrieval 353
      operationId: getAsset353
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 353
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_354:
    get:
      summary: Enterprise Resource Retrieval 354
      operationId: getAsset354
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 354
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_355:
    get:
      summary: Enterprise Resource Retrieval 355
      operationId: getAsset355
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 355
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_356:
    get:
      summary: Enterprise Resource Retrieval 356
      operationId: getAsset356
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 356
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_357:
    get:
      summary: Enterprise Resource Retrieval 357
      operationId: getAsset357
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 357
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_358:
    get:
      summary: Enterprise Resource Retrieval 358
      operationId: getAsset358
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 358
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_359:
    get:
      summary: Enterprise Resource Retrieval 359
      operationId: getAsset359
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 359
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_360:
    get:
      summary: Enterprise Resource Retrieval 360
      operationId: getAsset360
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 360
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_361:
    get:
      summary: Enterprise Resource Retrieval 361
      operationId: getAsset361
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 361
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_362:
    get:
      summary: Enterprise Resource Retrieval 362
      operationId: getAsset362
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 362
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_363:
    get:
      summary: Enterprise Resource Retrieval 363
      operationId: getAsset363
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 363
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_364:
    get:
      summary: Enterprise Resource Retrieval 364
      operationId: getAsset364
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 364
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_365:
    get:
      summary: Enterprise Resource Retrieval 365
      operationId: getAsset365
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 365
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_366:
    get:
      summary: Enterprise Resource Retrieval 366
      operationId: getAsset366
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 366
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_367:
    get:
      summary: Enterprise Resource Retrieval 367
      operationId: getAsset367
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 367
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_368:
    get:
      summary: Enterprise Resource Retrieval 368
      operationId: getAsset368
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 368
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_369:
    get:
      summary: Enterprise Resource Retrieval 369
      operationId: getAsset369
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 369
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_370:
    get:
      summary: Enterprise Resource Retrieval 370
      operationId: getAsset370
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 370
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_371:
    get:
      summary: Enterprise Resource Retrieval 371
      operationId: getAsset371
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 371
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_372:
    get:
      summary: Enterprise Resource Retrieval 372
      operationId: getAsset372
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 372
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_373:
    get:
      summary: Enterprise Resource Retrieval 373
      operationId: getAsset373
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 373
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_374:
    get:
      summary: Enterprise Resource Retrieval 374
      operationId: getAsset374
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 374
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_375:
    get:
      summary: Enterprise Resource Retrieval 375
      operationId: getAsset375
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 375
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_376:
    get:
      summary: Enterprise Resource Retrieval 376
      operationId: getAsset376
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 376
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_377:
    get:
      summary: Enterprise Resource Retrieval 377
      operationId: getAsset377
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 377
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_378:
    get:
      summary: Enterprise Resource Retrieval 378
      operationId: getAsset378
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 378
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_379:
    get:
      summary: Enterprise Resource Retrieval 379
      operationId: getAsset379
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 379
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_380:
    get:
      summary: Enterprise Resource Retrieval 380
      operationId: getAsset380
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 380
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_381:
    get:
      summary: Enterprise Resource Retrieval 381
      operationId: getAsset381
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 381
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_382:
    get:
      summary: Enterprise Resource Retrieval 382
      operationId: getAsset382
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 382
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_383:
    get:
      summary: Enterprise Resource Retrieval 383
      operationId: getAsset383
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 383
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_384:
    get:
      summary: Enterprise Resource Retrieval 384
      operationId: getAsset384
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 384
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_385:
    get:
      summary: Enterprise Resource Retrieval 385
      operationId: getAsset385
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 385
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_386:
    get:
      summary: Enterprise Resource Retrieval 386
      operationId: getAsset386
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 386
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_387:
    get:
      summary: Enterprise Resource Retrieval 387
      operationId: getAsset387
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 387
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_388:
    get:
      summary: Enterprise Resource Retrieval 388
      operationId: getAsset388
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 388
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_389:
    get:
      summary: Enterprise Resource Retrieval 389
      operationId: getAsset389
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 389
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_390:
    get:
      summary: Enterprise Resource Retrieval 390
      operationId: getAsset390
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 390
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_391:
    get:
      summary: Enterprise Resource Retrieval 391
      operationId: getAsset391
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 391
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_392:
    get:
      summary: Enterprise Resource Retrieval 392
      operationId: getAsset392
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 392
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_393:
    get:
      summary: Enterprise Resource Retrieval 393
      operationId: getAsset393
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 393
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_394:
    get:
      summary: Enterprise Resource Retrieval 394
      operationId: getAsset394
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 394
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_395:
    get:
      summary: Enterprise Resource Retrieval 395
      operationId: getAsset395
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 395
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_396:
    get:
      summary: Enterprise Resource Retrieval 396
      operationId: getAsset396
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 396
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_397:
    get:
      summary: Enterprise Resource Retrieval 397
      operationId: getAsset397
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 397
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_398:
    get:
      summary: Enterprise Resource Retrieval 398
      operationId: getAsset398
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 398
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_399:
    get:
      summary: Enterprise Resource Retrieval 399
      operationId: getAsset399
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 399
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_400:
    get:
      summary: Enterprise Resource Retrieval 400
      operationId: getAsset400
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 400
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_401:
    get:
      summary: Enterprise Resource Retrieval 401
      operationId: getAsset401
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 401
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_402:
    get:
      summary: Enterprise Resource Retrieval 402
      operationId: getAsset402
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 402
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_403:
    get:
      summary: Enterprise Resource Retrieval 403
      operationId: getAsset403
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 403
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_404:
    get:
      summary: Enterprise Resource Retrieval 404
      operationId: getAsset404
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 404
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_405:
    get:
      summary: Enterprise Resource Retrieval 405
      operationId: getAsset405
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 405
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_406:
    get:
      summary: Enterprise Resource Retrieval 406
      operationId: getAsset406
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 406
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_407:
    get:
      summary: Enterprise Resource Retrieval 407
      operationId: getAsset407
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 407
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_408:
    get:
      summary: Enterprise Resource Retrieval 408
      operationId: getAsset408
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 408
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_409:
    get:
      summary: Enterprise Resource Retrieval 409
      operationId: getAsset409
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 409
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_410:
    get:
      summary: Enterprise Resource Retrieval 410
      operationId: getAsset410
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 410
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_411:
    get:
      summary: Enterprise Resource Retrieval 411
      operationId: getAsset411
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 411
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_412:
    get:
      summary: Enterprise Resource Retrieval 412
      operationId: getAsset412
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 412
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_413:
    get:
      summary: Enterprise Resource Retrieval 413
      operationId: getAsset413
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 413
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_414:
    get:
      summary: Enterprise Resource Retrieval 414
      operationId: getAsset414
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 414
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_415:
    get:
      summary: Enterprise Resource Retrieval 415
      operationId: getAsset415
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 415
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_416:
    get:
      summary: Enterprise Resource Retrieval 416
      operationId: getAsset416
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 416
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_417:
    get:
      summary: Enterprise Resource Retrieval 417
      operationId: getAsset417
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 417
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_418:
    get:
      summary: Enterprise Resource Retrieval 418
      operationId: getAsset418
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 418
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_419:
    get:
      summary: Enterprise Resource Retrieval 419
      operationId: getAsset419
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 419
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_420:
    get:
      summary: Enterprise Resource Retrieval 420
      operationId: getAsset420
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 420
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_421:
    get:
      summary: Enterprise Resource Retrieval 421
      operationId: getAsset421
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 421
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_422:
    get:
      summary: Enterprise Resource Retrieval 422
      operationId: getAsset422
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 422
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_423:
    get:
      summary: Enterprise Resource Retrieval 423
      operationId: getAsset423
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 423
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_424:
    get:
      summary: Enterprise Resource Retrieval 424
      operationId: getAsset424
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 424
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_425:
    get:
      summary: Enterprise Resource Retrieval 425
      operationId: getAsset425
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 425
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_426:
    get:
      summary: Enterprise Resource Retrieval 426
      operationId: getAsset426
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 426
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_427:
    get:
      summary: Enterprise Resource Retrieval 427
      operationId: getAsset427
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 427
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_428:
    get:
      summary: Enterprise Resource Retrieval 428
      operationId: getAsset428
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 428
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_429:
    get:
      summary: Enterprise Resource Retrieval 429
      operationId: getAsset429
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 429
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_430:
    get:
      summary: Enterprise Resource Retrieval 430
      operationId: getAsset430
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 430
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_431:
    get:
      summary: Enterprise Resource Retrieval 431
      operationId: getAsset431
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 431
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_432:
    get:
      summary: Enterprise Resource Retrieval 432
      operationId: getAsset432
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 432
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_433:
    get:
      summary: Enterprise Resource Retrieval 433
      operationId: getAsset433
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 433
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_434:
    get:
      summary: Enterprise Resource Retrieval 434
      operationId: getAsset434
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 434
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_435:
    get:
      summary: Enterprise Resource Retrieval 435
      operationId: getAsset435
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 435
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_436:
    get:
      summary: Enterprise Resource Retrieval 436
      operationId: getAsset436
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 436
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_437:
    get:
      summary: Enterprise Resource Retrieval 437
      operationId: getAsset437
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 437
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_438:
    get:
      summary: Enterprise Resource Retrieval 438
      operationId: getAsset438
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 438
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_439:
    get:
      summary: Enterprise Resource Retrieval 439
      operationId: getAsset439
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 439
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_440:
    get:
      summary: Enterprise Resource Retrieval 440
      operationId: getAsset440
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 440
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_441:
    get:
      summary: Enterprise Resource Retrieval 441
      operationId: getAsset441
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 441
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_442:
    get:
      summary: Enterprise Resource Retrieval 442
      operationId: getAsset442
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 442
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_443:
    get:
      summary: Enterprise Resource Retrieval 443
      operationId: getAsset443
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 443
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_444:
    get:
      summary: Enterprise Resource Retrieval 444
      operationId: getAsset444
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 444
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_445:
    get:
      summary: Enterprise Resource Retrieval 445
      operationId: getAsset445
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 445
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_446:
    get:
      summary: Enterprise Resource Retrieval 446
      operationId: getAsset446
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 446
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_447:
    get:
      summary: Enterprise Resource Retrieval 447
      operationId: getAsset447
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 447
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_448:
    get:
      summary: Enterprise Resource Retrieval 448
      operationId: getAsset448
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 448
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_449:
    get:
      summary: Enterprise Resource Retrieval 449
      operationId: getAsset449
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 449
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_450:
    get:
      summary: Enterprise Resource Retrieval 450
      operationId: getAsset450
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 450
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_451:
    get:
      summary: Enterprise Resource Retrieval 451
      operationId: getAsset451
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 451
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_452:
    get:
      summary: Enterprise Resource Retrieval 452
      operationId: getAsset452
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 452
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_453:
    get:
      summary: Enterprise Resource Retrieval 453
      operationId: getAsset453
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 453
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_454:
    get:
      summary: Enterprise Resource Retrieval 454
      operationId: getAsset454
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 454
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_455:
    get:
      summary: Enterprise Resource Retrieval 455
      operationId: getAsset455
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 455
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_456:
    get:
      summary: Enterprise Resource Retrieval 456
      operationId: getAsset456
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 456
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_457:
    get:
      summary: Enterprise Resource Retrieval 457
      operationId: getAsset457
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 457
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_458:
    get:
      summary: Enterprise Resource Retrieval 458
      operationId: getAsset458
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 458
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_459:
    get:
      summary: Enterprise Resource Retrieval 459
      operationId: getAsset459
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 459
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_460:
    get:
      summary: Enterprise Resource Retrieval 460
      operationId: getAsset460
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 460
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_461:
    get:
      summary: Enterprise Resource Retrieval 461
      operationId: getAsset461
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 461
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_462:
    get:
      summary: Enterprise Resource Retrieval 462
      operationId: getAsset462
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 462
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_463:
    get:
      summary: Enterprise Resource Retrieval 463
      operationId: getAsset463
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 463
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_464:
    get:
      summary: Enterprise Resource Retrieval 464
      operationId: getAsset464
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 464
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_465:
    get:
      summary: Enterprise Resource Retrieval 465
      operationId: getAsset465
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 465
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_466:
    get:
      summary: Enterprise Resource Retrieval 466
      operationId: getAsset466
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 466
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_467:
    get:
      summary: Enterprise Resource Retrieval 467
      operationId: getAsset467
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 467
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_468:
    get:
      summary: Enterprise Resource Retrieval 468
      operationId: getAsset468
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 468
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_469:
    get:
      summary: Enterprise Resource Retrieval 469
      operationId: getAsset469
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 469
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_470:
    get:
      summary: Enterprise Resource Retrieval 470
      operationId: getAsset470
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 470
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_471:
    get:
      summary: Enterprise Resource Retrieval 471
      operationId: getAsset471
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 471
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_472:
    get:
      summary: Enterprise Resource Retrieval 472
      operationId: getAsset472
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 472
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_473:
    get:
      summary: Enterprise Resource Retrieval 473
      operationId: getAsset473
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 473
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_474:
    get:
      summary: Enterprise Resource Retrieval 474
      operationId: getAsset474
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 474
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_475:
    get:
      summary: Enterprise Resource Retrieval 475
      operationId: getAsset475
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 475
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_476:
    get:
      summary: Enterprise Resource Retrieval 476
      operationId: getAsset476
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 476
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_477:
    get:
      summary: Enterprise Resource Retrieval 477
      operationId: getAsset477
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 477
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_478:
    get:
      summary: Enterprise Resource Retrieval 478
      operationId: getAsset478
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 478
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_479:
    get:
      summary: Enterprise Resource Retrieval 479
      operationId: getAsset479
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 479
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_480:
    get:
      summary: Enterprise Resource Retrieval 480
      operationId: getAsset480
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 480
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_481:
    get:
      summary: Enterprise Resource Retrieval 481
      operationId: getAsset481
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 481
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_482:
    get:
      summary: Enterprise Resource Retrieval 482
      operationId: getAsset482
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 482
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_483:
    get:
      summary: Enterprise Resource Retrieval 483
      operationId: getAsset483
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 483
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_484:
    get:
      summary: Enterprise Resource Retrieval 484
      operationId: getAsset484
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 484
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_485:
    get:
      summary: Enterprise Resource Retrieval 485
      operationId: getAsset485
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 485
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_486:
    get:
      summary: Enterprise Resource Retrieval 486
      operationId: getAsset486
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 486
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_487:
    get:
      summary: Enterprise Resource Retrieval 487
      operationId: getAsset487
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 487
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_488:
    get:
      summary: Enterprise Resource Retrieval 488
      operationId: getAsset488
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 488
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_489:
    get:
      summary: Enterprise Resource Retrieval 489
      operationId: getAsset489
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 489
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_490:
    get:
      summary: Enterprise Resource Retrieval 490
      operationId: getAsset490
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 490
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_491:
    get:
      summary: Enterprise Resource Retrieval 491
      operationId: getAsset491
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 491
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_492:
    get:
      summary: Enterprise Resource Retrieval 492
      operationId: getAsset492
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 492
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_493:
    get:
      summary: Enterprise Resource Retrieval 493
      operationId: getAsset493
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 493
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_494:
    get:
      summary: Enterprise Resource Retrieval 494
      operationId: getAsset494
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 494
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_495:
    get:
      summary: Enterprise Resource Retrieval 495
      operationId: getAsset495
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 495
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_496:
    get:
      summary: Enterprise Resource Retrieval 496
      operationId: getAsset496
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 496
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_497:
    get:
      summary: Enterprise Resource Retrieval 497
      operationId: getAsset497
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 497
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_498:
    get:
      summary: Enterprise Resource Retrieval 498
      operationId: getAsset498
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 498
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_499:
    get:
      summary: Enterprise Resource Retrieval 499
      operationId: getAsset499
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 499
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_500:
    get:
      summary: Enterprise Resource Retrieval 500
      operationId: getAsset500
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 500
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_501:
    get:
      summary: Enterprise Resource Retrieval 501
      operationId: getAsset501
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 501
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_502:
    get:
      summary: Enterprise Resource Retrieval 502
      operationId: getAsset502
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 502
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_503:
    get:
      summary: Enterprise Resource Retrieval 503
      operationId: getAsset503
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 503
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_504:
    get:
      summary: Enterprise Resource Retrieval 504
      operationId: getAsset504
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 504
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_505:
    get:
      summary: Enterprise Resource Retrieval 505
      operationId: getAsset505
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 505
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_506:
    get:
      summary: Enterprise Resource Retrieval 506
      operationId: getAsset506
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 506
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_507:
    get:
      summary: Enterprise Resource Retrieval 507
      operationId: getAsset507
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 507
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_508:
    get:
      summary: Enterprise Resource Retrieval 508
      operationId: getAsset508
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 508
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_509:
    get:
      summary: Enterprise Resource Retrieval 509
      operationId: getAsset509
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 509
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_510:
    get:
      summary: Enterprise Resource Retrieval 510
      operationId: getAsset510
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 510
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_511:
    get:
      summary: Enterprise Resource Retrieval 511
      operationId: getAsset511
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 511
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_512:
    get:
      summary: Enterprise Resource Retrieval 512
      operationId: getAsset512
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 512
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_513:
    get:
      summary: Enterprise Resource Retrieval 513
      operationId: getAsset513
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 513
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_514:
    get:
      summary: Enterprise Resource Retrieval 514
      operationId: getAsset514
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 514
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_515:
    get:
      summary: Enterprise Resource Retrieval 515
      operationId: getAsset515
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 515
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_516:
    get:
      summary: Enterprise Resource Retrieval 516
      operationId: getAsset516
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 516
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_517:
    get:
      summary: Enterprise Resource Retrieval 517
      operationId: getAsset517
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 517
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_518:
    get:
      summary: Enterprise Resource Retrieval 518
      operationId: getAsset518
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 518
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_519:
    get:
      summary: Enterprise Resource Retrieval 519
      operationId: getAsset519
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 519
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_520:
    get:
      summary: Enterprise Resource Retrieval 520
      operationId: getAsset520
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 520
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_521:
    get:
      summary: Enterprise Resource Retrieval 521
      operationId: getAsset521
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 521
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_522:
    get:
      summary: Enterprise Resource Retrieval 522
      operationId: getAsset522
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 522
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_523:
    get:
      summary: Enterprise Resource Retrieval 523
      operationId: getAsset523
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 523
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_524:
    get:
      summary: Enterprise Resource Retrieval 524
      operationId: getAsset524
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 524
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_525:
    get:
      summary: Enterprise Resource Retrieval 525
      operationId: getAsset525
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 525
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_526:
    get:
      summary: Enterprise Resource Retrieval 526
      operationId: getAsset526
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 526
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_527:
    get:
      summary: Enterprise Resource Retrieval 527
      operationId: getAsset527
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 527
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_528:
    get:
      summary: Enterprise Resource Retrieval 528
      operationId: getAsset528
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 528
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_529:
    get:
      summary: Enterprise Resource Retrieval 529
      operationId: getAsset529
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 529
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_530:
    get:
      summary: Enterprise Resource Retrieval 530
      operationId: getAsset530
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 530
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_531:
    get:
      summary: Enterprise Resource Retrieval 531
      operationId: getAsset531
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 531
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_532:
    get:
      summary: Enterprise Resource Retrieval 532
      operationId: getAsset532
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 532
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_533:
    get:
      summary: Enterprise Resource Retrieval 533
      operationId: getAsset533
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 533
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_534:
    get:
      summary: Enterprise Resource Retrieval 534
      operationId: getAsset534
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 534
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_535:
    get:
      summary: Enterprise Resource Retrieval 535
      operationId: getAsset535
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 535
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_536:
    get:
      summary: Enterprise Resource Retrieval 536
      operationId: getAsset536
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 536
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_537:
    get:
      summary: Enterprise Resource Retrieval 537
      operationId: getAsset537
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 537
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_538:
    get:
      summary: Enterprise Resource Retrieval 538
      operationId: getAsset538
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 538
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_539:
    get:
      summary: Enterprise Resource Retrieval 539
      operationId: getAsset539
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 539
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_540:
    get:
      summary: Enterprise Resource Retrieval 540
      operationId: getAsset540
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 540
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_541:
    get:
      summary: Enterprise Resource Retrieval 541
      operationId: getAsset541
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 541
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_542:
    get:
      summary: Enterprise Resource Retrieval 542
      operationId: getAsset542
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 542
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_543:
    get:
      summary: Enterprise Resource Retrieval 543
      operationId: getAsset543
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 543
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_544:
    get:
      summary: Enterprise Resource Retrieval 544
      operationId: getAsset544
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 544
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_545:
    get:
      summary: Enterprise Resource Retrieval 545
      operationId: getAsset545
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 545
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_546:
    get:
      summary: Enterprise Resource Retrieval 546
      operationId: getAsset546
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 546
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_547:
    get:
      summary: Enterprise Resource Retrieval 547
      operationId: getAsset547
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 547
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_548:
    get:
      summary: Enterprise Resource Retrieval 548
      operationId: getAsset548
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 548
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_549:
    get:
      summary: Enterprise Resource Retrieval 549
      operationId: getAsset549
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 549
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_550:
    get:
      summary: Enterprise Resource Retrieval 550
      operationId: getAsset550
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 550
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_551:
    get:
      summary: Enterprise Resource Retrieval 551
      operationId: getAsset551
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 551
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_552:
    get:
      summary: Enterprise Resource Retrieval 552
      operationId: getAsset552
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 552
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_553:
    get:
      summary: Enterprise Resource Retrieval 553
      operationId: getAsset553
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 553
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_554:
    get:
      summary: Enterprise Resource Retrieval 554
      operationId: getAsset554
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 554
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_555:
    get:
      summary: Enterprise Resource Retrieval 555
      operationId: getAsset555
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 555
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_556:
    get:
      summary: Enterprise Resource Retrieval 556
      operationId: getAsset556
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 556
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_557:
    get:
      summary: Enterprise Resource Retrieval 557
      operationId: getAsset557
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 557
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_558:
    get:
      summary: Enterprise Resource Retrieval 558
      operationId: getAsset558
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 558
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_559:
    get:
      summary: Enterprise Resource Retrieval 559
      operationId: getAsset559
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 559
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_560:
    get:
      summary: Enterprise Resource Retrieval 560
      operationId: getAsset560
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 560
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_561:
    get:
      summary: Enterprise Resource Retrieval 561
      operationId: getAsset561
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 561
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_562:
    get:
      summary: Enterprise Resource Retrieval 562
      operationId: getAsset562
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 562
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_563:
    get:
      summary: Enterprise Resource Retrieval 563
      operationId: getAsset563
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 563
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_564:
    get:
      summary: Enterprise Resource Retrieval 564
      operationId: getAsset564
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 564
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_565:
    get:
      summary: Enterprise Resource Retrieval 565
      operationId: getAsset565
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 565
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_566:
    get:
      summary: Enterprise Resource Retrieval 566
      operationId: getAsset566
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 566
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_567:
    get:
      summary: Enterprise Resource Retrieval 567
      operationId: getAsset567
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 567
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_568:
    get:
      summary: Enterprise Resource Retrieval 568
      operationId: getAsset568
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 568
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_569:
    get:
      summary: Enterprise Resource Retrieval 569
      operationId: getAsset569
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 569
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_570:
    get:
      summary: Enterprise Resource Retrieval 570
      operationId: getAsset570
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 570
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_571:
    get:
      summary: Enterprise Resource Retrieval 571
      operationId: getAsset571
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 571
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_572:
    get:
      summary: Enterprise Resource Retrieval 572
      operationId: getAsset572
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 572
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_573:
    get:
      summary: Enterprise Resource Retrieval 573
      operationId: getAsset573
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 573
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_574:
    get:
      summary: Enterprise Resource Retrieval 574
      operationId: getAsset574
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 574
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_575:
    get:
      summary: Enterprise Resource Retrieval 575
      operationId: getAsset575
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 575
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_576:
    get:
      summary: Enterprise Resource Retrieval 576
      operationId: getAsset576
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 576
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_577:
    get:
      summary: Enterprise Resource Retrieval 577
      operationId: getAsset577
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 577
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_578:
    get:
      summary: Enterprise Resource Retrieval 578
      operationId: getAsset578
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 578
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_579:
    get:
      summary: Enterprise Resource Retrieval 579
      operationId: getAsset579
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 579
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_580:
    get:
      summary: Enterprise Resource Retrieval 580
      operationId: getAsset580
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 580
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_581:
    get:
      summary: Enterprise Resource Retrieval 581
      operationId: getAsset581
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 581
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_582:
    get:
      summary: Enterprise Resource Retrieval 582
      operationId: getAsset582
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 582
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_583:
    get:
      summary: Enterprise Resource Retrieval 583
      operationId: getAsset583
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 583
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_584:
    get:
      summary: Enterprise Resource Retrieval 584
      operationId: getAsset584
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 584
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_585:
    get:
      summary: Enterprise Resource Retrieval 585
      operationId: getAsset585
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 585
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_586:
    get:
      summary: Enterprise Resource Retrieval 586
      operationId: getAsset586
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 586
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_587:
    get:
      summary: Enterprise Resource Retrieval 587
      operationId: getAsset587
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 587
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_588:
    get:
      summary: Enterprise Resource Retrieval 588
      operationId: getAsset588
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 588
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_589:
    get:
      summary: Enterprise Resource Retrieval 589
      operationId: getAsset589
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 589
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_590:
    get:
      summary: Enterprise Resource Retrieval 590
      operationId: getAsset590
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 590
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_591:
    get:
      summary: Enterprise Resource Retrieval 591
      operationId: getAsset591
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 591
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_592:
    get:
      summary: Enterprise Resource Retrieval 592
      operationId: getAsset592
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 592
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_593:
    get:
      summary: Enterprise Resource Retrieval 593
      operationId: getAsset593
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 593
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_594:
    get:
      summary: Enterprise Resource Retrieval 594
      operationId: getAsset594
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 594
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_595:
    get:
      summary: Enterprise Resource Retrieval 595
      operationId: getAsset595
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 595
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_596:
    get:
      summary: Enterprise Resource Retrieval 596
      operationId: getAsset596
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 596
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_597:
    get:
      summary: Enterprise Resource Retrieval 597
      operationId: getAsset597
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 597
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_598:
    get:
      summary: Enterprise Resource Retrieval 598
      operationId: getAsset598
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 598
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_599:
    get:
      summary: Enterprise Resource Retrieval 599
      operationId: getAsset599
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 599
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_600:
    get:
      summary: Enterprise Resource Retrieval 600
      operationId: getAsset600
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 600
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_601:
    get:
      summary: Enterprise Resource Retrieval 601
      operationId: getAsset601
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 601
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_602:
    get:
      summary: Enterprise Resource Retrieval 602
      operationId: getAsset602
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 602
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_603:
    get:
      summary: Enterprise Resource Retrieval 603
      operationId: getAsset603
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 603
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_604:
    get:
      summary: Enterprise Resource Retrieval 604
      operationId: getAsset604
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 604
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_605:
    get:
      summary: Enterprise Resource Retrieval 605
      operationId: getAsset605
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 605
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_606:
    get:
      summary: Enterprise Resource Retrieval 606
      operationId: getAsset606
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 606
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_607:
    get:
      summary: Enterprise Resource Retrieval 607
      operationId: getAsset607
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 607
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_608:
    get:
      summary: Enterprise Resource Retrieval 608
      operationId: getAsset608
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 608
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_609:
    get:
      summary: Enterprise Resource Retrieval 609
      operationId: getAsset609
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 609
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_610:
    get:
      summary: Enterprise Resource Retrieval 610
      operationId: getAsset610
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 610
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_611:
    get:
      summary: Enterprise Resource Retrieval 611
      operationId: getAsset611
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 611
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_612:
    get:
      summary: Enterprise Resource Retrieval 612
      operationId: getAsset612
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 612
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_613:
    get:
      summary: Enterprise Resource Retrieval 613
      operationId: getAsset613
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 613
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_614:
    get:
      summary: Enterprise Resource Retrieval 614
      operationId: getAsset614
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 614
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_615:
    get:
      summary: Enterprise Resource Retrieval 615
      operationId: getAsset615
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 615
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_616:
    get:
      summary: Enterprise Resource Retrieval 616
      operationId: getAsset616
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 616
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_617:
    get:
      summary: Enterprise Resource Retrieval 617
      operationId: getAsset617
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 617
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_618:
    get:
      summary: Enterprise Resource Retrieval 618
      operationId: getAsset618
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 618
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_619:
    get:
      summary: Enterprise Resource Retrieval 619
      operationId: getAsset619
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 619
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_620:
    get:
      summary: Enterprise Resource Retrieval 620
      operationId: getAsset620
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 620
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_621:
    get:
      summary: Enterprise Resource Retrieval 621
      operationId: getAsset621
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 621
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_622:
    get:
      summary: Enterprise Resource Retrieval 622
      operationId: getAsset622
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 622
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_623:
    get:
      summary: Enterprise Resource Retrieval 623
      operationId: getAsset623
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 623
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_624:
    get:
      summary: Enterprise Resource Retrieval 624
      operationId: getAsset624
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 624
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_625:
    get:
      summary: Enterprise Resource Retrieval 625
      operationId: getAsset625
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 625
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_626:
    get:
      summary: Enterprise Resource Retrieval 626
      operationId: getAsset626
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 626
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_627:
    get:
      summary: Enterprise Resource Retrieval 627
      operationId: getAsset627
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 627
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_628:
    get:
      summary: Enterprise Resource Retrieval 628
      operationId: getAsset628
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 628
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_629:
    get:
      summary: Enterprise Resource Retrieval 629
      operationId: getAsset629
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 629
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_630:
    get:
      summary: Enterprise Resource Retrieval 630
      operationId: getAsset630
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 630
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_631:
    get:
      summary: Enterprise Resource Retrieval 631
      operationId: getAsset631
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 631
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_632:
    get:
      summary: Enterprise Resource Retrieval 632
      operationId: getAsset632
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 632
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_633:
    get:
      summary: Enterprise Resource Retrieval 633
      operationId: getAsset633
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 633
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_634:
    get:
      summary: Enterprise Resource Retrieval 634
      operationId: getAsset634
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 634
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_635:
    get:
      summary: Enterprise Resource Retrieval 635
      operationId: getAsset635
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 635
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_636:
    get:
      summary: Enterprise Resource Retrieval 636
      operationId: getAsset636
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 636
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_637:
    get:
      summary: Enterprise Resource Retrieval 637
      operationId: getAsset637
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 637
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_638:
    get:
      summary: Enterprise Resource Retrieval 638
      operationId: getAsset638
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 638
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_639:
    get:
      summary: Enterprise Resource Retrieval 639
      operationId: getAsset639
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 639
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_640:
    get:
      summary: Enterprise Resource Retrieval 640
      operationId: getAsset640
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 640
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_641:
    get:
      summary: Enterprise Resource Retrieval 641
      operationId: getAsset641
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 641
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_642:
    get:
      summary: Enterprise Resource Retrieval 642
      operationId: getAsset642
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 642
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_643:
    get:
      summary: Enterprise Resource Retrieval 643
      operationId: getAsset643
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 643
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_644:
    get:
      summary: Enterprise Resource Retrieval 644
      operationId: getAsset644
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 644
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_645:
    get:
      summary: Enterprise Resource Retrieval 645
      operationId: getAsset645
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 645
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_646:
    get:
      summary: Enterprise Resource Retrieval 646
      operationId: getAsset646
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 646
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_647:
    get:
      summary: Enterprise Resource Retrieval 647
      operationId: getAsset647
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 647
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_648:
    get:
      summary: Enterprise Resource Retrieval 648
      operationId: getAsset648
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 648
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_649:
    get:
      summary: Enterprise Resource Retrieval 649
      operationId: getAsset649
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 649
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_650:
    get:
      summary: Enterprise Resource Retrieval 650
      operationId: getAsset650
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 650
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_651:
    get:
      summary: Enterprise Resource Retrieval 651
      operationId: getAsset651
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 651
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_652:
    get:
      summary: Enterprise Resource Retrieval 652
      operationId: getAsset652
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 652
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_653:
    get:
      summary: Enterprise Resource Retrieval 653
      operationId: getAsset653
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 653
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_654:
    get:
      summary: Enterprise Resource Retrieval 654
      operationId: getAsset654
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 654
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_655:
    get:
      summary: Enterprise Resource Retrieval 655
      operationId: getAsset655
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 655
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_656:
    get:
      summary: Enterprise Resource Retrieval 656
      operationId: getAsset656
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 656
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_657:
    get:
      summary: Enterprise Resource Retrieval 657
      operationId: getAsset657
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 657
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_658:
    get:
      summary: Enterprise Resource Retrieval 658
      operationId: getAsset658
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 658
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_659:
    get:
      summary: Enterprise Resource Retrieval 659
      operationId: getAsset659
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 659
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_660:
    get:
      summary: Enterprise Resource Retrieval 660
      operationId: getAsset660
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 660
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_661:
    get:
      summary: Enterprise Resource Retrieval 661
      operationId: getAsset661
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 661
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_662:
    get:
      summary: Enterprise Resource Retrieval 662
      operationId: getAsset662
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 662
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_663:
    get:
      summary: Enterprise Resource Retrieval 663
      operationId: getAsset663
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 663
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_664:
    get:
      summary: Enterprise Resource Retrieval 664
      operationId: getAsset664
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 664
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_665:
    get:
      summary: Enterprise Resource Retrieval 665
      operationId: getAsset665
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 665
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_666:
    get:
      summary: Enterprise Resource Retrieval 666
      operationId: getAsset666
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 666
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_667:
    get:
      summary: Enterprise Resource Retrieval 667
      operationId: getAsset667
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 667
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_668:
    get:
      summary: Enterprise Resource Retrieval 668
      operationId: getAsset668
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 668
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_669:
    get:
      summary: Enterprise Resource Retrieval 669
      operationId: getAsset669
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 669
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_670:
    get:
      summary: Enterprise Resource Retrieval 670
      operationId: getAsset670
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 670
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_671:
    get:
      summary: Enterprise Resource Retrieval 671
      operationId: getAsset671
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 671
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_672:
    get:
      summary: Enterprise Resource Retrieval 672
      operationId: getAsset672
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 672
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_673:
    get:
      summary: Enterprise Resource Retrieval 673
      operationId: getAsset673
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 673
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_674:
    get:
      summary: Enterprise Resource Retrieval 674
      operationId: getAsset674
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 674
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_675:
    get:
      summary: Enterprise Resource Retrieval 675
      operationId: getAsset675
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 675
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_676:
    get:
      summary: Enterprise Resource Retrieval 676
      operationId: getAsset676
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 676
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_677:
    get:
      summary: Enterprise Resource Retrieval 677
      operationId: getAsset677
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 677
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_678:
    get:
      summary: Enterprise Resource Retrieval 678
      operationId: getAsset678
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 678
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_679:
    get:
      summary: Enterprise Resource Retrieval 679
      operationId: getAsset679
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 679
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_680:
    get:
      summary: Enterprise Resource Retrieval 680
      operationId: getAsset680
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 680
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_681:
    get:
      summary: Enterprise Resource Retrieval 681
      operationId: getAsset681
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 681
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_682:
    get:
      summary: Enterprise Resource Retrieval 682
      operationId: getAsset682
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 682
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_683:
    get:
      summary: Enterprise Resource Retrieval 683
      operationId: getAsset683
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 683
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_684:
    get:
      summary: Enterprise Resource Retrieval 684
      operationId: getAsset684
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 684
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_685:
    get:
      summary: Enterprise Resource Retrieval 685
      operationId: getAsset685
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 685
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_686:
    get:
      summary: Enterprise Resource Retrieval 686
      operationId: getAsset686
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 686
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_687:
    get:
      summary: Enterprise Resource Retrieval 687
      operationId: getAsset687
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 687
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_688:
    get:
      summary: Enterprise Resource Retrieval 688
      operationId: getAsset688
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 688
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_689:
    get:
      summary: Enterprise Resource Retrieval 689
      operationId: getAsset689
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 689
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_690:
    get:
      summary: Enterprise Resource Retrieval 690
      operationId: getAsset690
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 690
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_691:
    get:
      summary: Enterprise Resource Retrieval 691
      operationId: getAsset691
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 691
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_692:
    get:
      summary: Enterprise Resource Retrieval 692
      operationId: getAsset692
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 692
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_693:
    get:
      summary: Enterprise Resource Retrieval 693
      operationId: getAsset693
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 693
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_694:
    get:
      summary: Enterprise Resource Retrieval 694
      operationId: getAsset694
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 694
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_695:
    get:
      summary: Enterprise Resource Retrieval 695
      operationId: getAsset695
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 695
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_696:
    get:
      summary: Enterprise Resource Retrieval 696
      operationId: getAsset696
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 696
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_697:
    get:
      summary: Enterprise Resource Retrieval 697
      operationId: getAsset697
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 697
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_698:
    get:
      summary: Enterprise Resource Retrieval 698
      operationId: getAsset698
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 698
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_699:
    get:
      summary: Enterprise Resource Retrieval 699
      operationId: getAsset699
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 699
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_700:
    get:
      summary: Enterprise Resource Retrieval 700
      operationId: getAsset700
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 700
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_701:
    get:
      summary: Enterprise Resource Retrieval 701
      operationId: getAsset701
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 701
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_702:
    get:
      summary: Enterprise Resource Retrieval 702
      operationId: getAsset702
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 702
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_703:
    get:
      summary: Enterprise Resource Retrieval 703
      operationId: getAsset703
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 703
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_704:
    get:
      summary: Enterprise Resource Retrieval 704
      operationId: getAsset704
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 704
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_705:
    get:
      summary: Enterprise Resource Retrieval 705
      operationId: getAsset705
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 705
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_706:
    get:
      summary: Enterprise Resource Retrieval 706
      operationId: getAsset706
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 706
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_707:
    get:
      summary: Enterprise Resource Retrieval 707
      operationId: getAsset707
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 707
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_708:
    get:
      summary: Enterprise Resource Retrieval 708
      operationId: getAsset708
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 708
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_709:
    get:
      summary: Enterprise Resource Retrieval 709
      operationId: getAsset709
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 709
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_710:
    get:
      summary: Enterprise Resource Retrieval 710
      operationId: getAsset710
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 710
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_711:
    get:
      summary: Enterprise Resource Retrieval 711
      operationId: getAsset711
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 711
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_712:
    get:
      summary: Enterprise Resource Retrieval 712
      operationId: getAsset712
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 712
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_713:
    get:
      summary: Enterprise Resource Retrieval 713
      operationId: getAsset713
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 713
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_714:
    get:
      summary: Enterprise Resource Retrieval 714
      operationId: getAsset714
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 714
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_715:
    get:
      summary: Enterprise Resource Retrieval 715
      operationId: getAsset715
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 715
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_716:
    get:
      summary: Enterprise Resource Retrieval 716
      operationId: getAsset716
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 716
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_717:
    get:
      summary: Enterprise Resource Retrieval 717
      operationId: getAsset717
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 717
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_718:
    get:
      summary: Enterprise Resource Retrieval 718
      operationId: getAsset718
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 718
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_719:
    get:
      summary: Enterprise Resource Retrieval 719
      operationId: getAsset719
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 719
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_720:
    get:
      summary: Enterprise Resource Retrieval 720
      operationId: getAsset720
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 720
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_721:
    get:
      summary: Enterprise Resource Retrieval 721
      operationId: getAsset721
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 721
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_722:
    get:
      summary: Enterprise Resource Retrieval 722
      operationId: getAsset722
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 722
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_723:
    get:
      summary: Enterprise Resource Retrieval 723
      operationId: getAsset723
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 723
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_724:
    get:
      summary: Enterprise Resource Retrieval 724
      operationId: getAsset724
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 724
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_725:
    get:
      summary: Enterprise Resource Retrieval 725
      operationId: getAsset725
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 725
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_726:
    get:
      summary: Enterprise Resource Retrieval 726
      operationId: getAsset726
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 726
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_727:
    get:
      summary: Enterprise Resource Retrieval 727
      operationId: getAsset727
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 727
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_728:
    get:
      summary: Enterprise Resource Retrieval 728
      operationId: getAsset728
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 728
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_729:
    get:
      summary: Enterprise Resource Retrieval 729
      operationId: getAsset729
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 729
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_730:
    get:
      summary: Enterprise Resource Retrieval 730
      operationId: getAsset730
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 730
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_731:
    get:
      summary: Enterprise Resource Retrieval 731
      operationId: getAsset731
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 731
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_732:
    get:
      summary: Enterprise Resource Retrieval 732
      operationId: getAsset732
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 732
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_733:
    get:
      summary: Enterprise Resource Retrieval 733
      operationId: getAsset733
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 733
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_734:
    get:
      summary: Enterprise Resource Retrieval 734
      operationId: getAsset734
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 734
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_735:
    get:
      summary: Enterprise Resource Retrieval 735
      operationId: getAsset735
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 735
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_736:
    get:
      summary: Enterprise Resource Retrieval 736
      operationId: getAsset736
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 736
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_737:
    get:
      summary: Enterprise Resource Retrieval 737
      operationId: getAsset737
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 737
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_738:
    get:
      summary: Enterprise Resource Retrieval 738
      operationId: getAsset738
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 738
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_739:
    get:
      summary: Enterprise Resource Retrieval 739
      operationId: getAsset739
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 739
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_740:
    get:
      summary: Enterprise Resource Retrieval 740
      operationId: getAsset740
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 740
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_741:
    get:
      summary: Enterprise Resource Retrieval 741
      operationId: getAsset741
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 741
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_742:
    get:
      summary: Enterprise Resource Retrieval 742
      operationId: getAsset742
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 742
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_743:
    get:
      summary: Enterprise Resource Retrieval 743
      operationId: getAsset743
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 743
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_744:
    get:
      summary: Enterprise Resource Retrieval 744
      operationId: getAsset744
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 744
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_745:
    get:
      summary: Enterprise Resource Retrieval 745
      operationId: getAsset745
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 745
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_746:
    get:
      summary: Enterprise Resource Retrieval 746
      operationId: getAsset746
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 746
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_747:
    get:
      summary: Enterprise Resource Retrieval 747
      operationId: getAsset747
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 747
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_748:
    get:
      summary: Enterprise Resource Retrieval 748
      operationId: getAsset748
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 748
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_749:
    get:
      summary: Enterprise Resource Retrieval 749
      operationId: getAsset749
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 749
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_750:
    get:
      summary: Enterprise Resource Retrieval 750
      operationId: getAsset750
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 750
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_751:
    get:
      summary: Enterprise Resource Retrieval 751
      operationId: getAsset751
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 751
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_752:
    get:
      summary: Enterprise Resource Retrieval 752
      operationId: getAsset752
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 752
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_753:
    get:
      summary: Enterprise Resource Retrieval 753
      operationId: getAsset753
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 753
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_754:
    get:
      summary: Enterprise Resource Retrieval 754
      operationId: getAsset754
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 754
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_755:
    get:
      summary: Enterprise Resource Retrieval 755
      operationId: getAsset755
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 755
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_756:
    get:
      summary: Enterprise Resource Retrieval 756
      operationId: getAsset756
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 756
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_757:
    get:
      summary: Enterprise Resource Retrieval 757
      operationId: getAsset757
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 757
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_758:
    get:
      summary: Enterprise Resource Retrieval 758
      operationId: getAsset758
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 758
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_759:
    get:
      summary: Enterprise Resource Retrieval 759
      operationId: getAsset759
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 759
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_760:
    get:
      summary: Enterprise Resource Retrieval 760
      operationId: getAsset760
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 760
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_761:
    get:
      summary: Enterprise Resource Retrieval 761
      operationId: getAsset761
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 761
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_762:
    get:
      summary: Enterprise Resource Retrieval 762
      operationId: getAsset762
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 762
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_763:
    get:
      summary: Enterprise Resource Retrieval 763
      operationId: getAsset763
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 763
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_764:
    get:
      summary: Enterprise Resource Retrieval 764
      operationId: getAsset764
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 764
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_765:
    get:
      summary: Enterprise Resource Retrieval 765
      operationId: getAsset765
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 765
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_766:
    get:
      summary: Enterprise Resource Retrieval 766
      operationId: getAsset766
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 766
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_767:
    get:
      summary: Enterprise Resource Retrieval 767
      operationId: getAsset767
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 767
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_768:
    get:
      summary: Enterprise Resource Retrieval 768
      operationId: getAsset768
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 768
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_769:
    get:
      summary: Enterprise Resource Retrieval 769
      operationId: getAsset769
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 769
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_770:
    get:
      summary: Enterprise Resource Retrieval 770
      operationId: getAsset770
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 770
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_771:
    get:
      summary: Enterprise Resource Retrieval 771
      operationId: getAsset771
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 771
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_772:
    get:
      summary: Enterprise Resource Retrieval 772
      operationId: getAsset772
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 772
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_773:
    get:
      summary: Enterprise Resource Retrieval 773
      operationId: getAsset773
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 773
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_774:
    get:
      summary: Enterprise Resource Retrieval 774
      operationId: getAsset774
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 774
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_775:
    get:
      summary: Enterprise Resource Retrieval 775
      operationId: getAsset775
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 775
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_776:
    get:
      summary: Enterprise Resource Retrieval 776
      operationId: getAsset776
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 776
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_777:
    get:
      summary: Enterprise Resource Retrieval 777
      operationId: getAsset777
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 777
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_778:
    get:
      summary: Enterprise Resource Retrieval 778
      operationId: getAsset778
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 778
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_779:
    get:
      summary: Enterprise Resource Retrieval 779
      operationId: getAsset779
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 779
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_780:
    get:
      summary: Enterprise Resource Retrieval 780
      operationId: getAsset780
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 780
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_781:
    get:
      summary: Enterprise Resource Retrieval 781
      operationId: getAsset781
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 781
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_782:
    get:
      summary: Enterprise Resource Retrieval 782
      operationId: getAsset782
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 782
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_783:
    get:
      summary: Enterprise Resource Retrieval 783
      operationId: getAsset783
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 783
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_784:
    get:
      summary: Enterprise Resource Retrieval 784
      operationId: getAsset784
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 784
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_785:
    get:
      summary: Enterprise Resource Retrieval 785
      operationId: getAsset785
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 785
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_786:
    get:
      summary: Enterprise Resource Retrieval 786
      operationId: getAsset786
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 786
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_787:
    get:
      summary: Enterprise Resource Retrieval 787
      operationId: getAsset787
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 787
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_788:
    get:
      summary: Enterprise Resource Retrieval 788
      operationId: getAsset788
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 788
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_789:
    get:
      summary: Enterprise Resource Retrieval 789
      operationId: getAsset789
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 789
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_790:
    get:
      summary: Enterprise Resource Retrieval 790
      operationId: getAsset790
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 790
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_791:
    get:
      summary: Enterprise Resource Retrieval 791
      operationId: getAsset791
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 791
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_792:
    get:
      summary: Enterprise Resource Retrieval 792
      operationId: getAsset792
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 792
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_793:
    get:
      summary: Enterprise Resource Retrieval 793
      operationId: getAsset793
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 793
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_794:
    get:
      summary: Enterprise Resource Retrieval 794
      operationId: getAsset794
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 794
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_795:
    get:
      summary: Enterprise Resource Retrieval 795
      operationId: getAsset795
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 795
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_796:
    get:
      summary: Enterprise Resource Retrieval 796
      operationId: getAsset796
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 796
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_797:
    get:
      summary: Enterprise Resource Retrieval 797
      operationId: getAsset797
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 797
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_798:
    get:
      summary: Enterprise Resource Retrieval 798
      operationId: getAsset798
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 798
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_799:
    get:
      summary: Enterprise Resource Retrieval 799
      operationId: getAsset799
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 799
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_800:
    get:
      summary: Enterprise Resource Retrieval 800
      operationId: getAsset800
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 800
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_801:
    get:
      summary: Enterprise Resource Retrieval 801
      operationId: getAsset801
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 801
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_802:
    get:
      summary: Enterprise Resource Retrieval 802
      operationId: getAsset802
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 802
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_803:
    get:
      summary: Enterprise Resource Retrieval 803
      operationId: getAsset803
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 803
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_804:
    get:
      summary: Enterprise Resource Retrieval 804
      operationId: getAsset804
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 804
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_805:
    get:
      summary: Enterprise Resource Retrieval 805
      operationId: getAsset805
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 805
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_806:
    get:
      summary: Enterprise Resource Retrieval 806
      operationId: getAsset806
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 806
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_807:
    get:
      summary: Enterprise Resource Retrieval 807
      operationId: getAsset807
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 807
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_808:
    get:
      summary: Enterprise Resource Retrieval 808
      operationId: getAsset808
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 808
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_809:
    get:
      summary: Enterprise Resource Retrieval 809
      operationId: getAsset809
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 809
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_810:
    get:
      summary: Enterprise Resource Retrieval 810
      operationId: getAsset810
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 810
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_811:
    get:
      summary: Enterprise Resource Retrieval 811
      operationId: getAsset811
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 811
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_812:
    get:
      summary: Enterprise Resource Retrieval 812
      operationId: getAsset812
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 812
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_813:
    get:
      summary: Enterprise Resource Retrieval 813
      operationId: getAsset813
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 813
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_814:
    get:
      summary: Enterprise Resource Retrieval 814
      operationId: getAsset814
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 814
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_815:
    get:
      summary: Enterprise Resource Retrieval 815
      operationId: getAsset815
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 815
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_816:
    get:
      summary: Enterprise Resource Retrieval 816
      operationId: getAsset816
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 816
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_817:
    get:
      summary: Enterprise Resource Retrieval 817
      operationId: getAsset817
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 817
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_818:
    get:
      summary: Enterprise Resource Retrieval 818
      operationId: getAsset818
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 818
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_819:
    get:
      summary: Enterprise Resource Retrieval 819
      operationId: getAsset819
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 819
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_820:
    get:
      summary: Enterprise Resource Retrieval 820
      operationId: getAsset820
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 820
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_821:
    get:
      summary: Enterprise Resource Retrieval 821
      operationId: getAsset821
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 821
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_822:
    get:
      summary: Enterprise Resource Retrieval 822
      operationId: getAsset822
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 822
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_823:
    get:
      summary: Enterprise Resource Retrieval 823
      operationId: getAsset823
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 823
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_824:
    get:
      summary: Enterprise Resource Retrieval 824
      operationId: getAsset824
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 824
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_825:
    get:
      summary: Enterprise Resource Retrieval 825
      operationId: getAsset825
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 825
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_826:
    get:
      summary: Enterprise Resource Retrieval 826
      operationId: getAsset826
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 826
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_827:
    get:
      summary: Enterprise Resource Retrieval 827
      operationId: getAsset827
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 827
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_828:
    get:
      summary: Enterprise Resource Retrieval 828
      operationId: getAsset828
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 828
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_829:
    get:
      summary: Enterprise Resource Retrieval 829
      operationId: getAsset829
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 829
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_830:
    get:
      summary: Enterprise Resource Retrieval 830
      operationId: getAsset830
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 830
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_831:
    get:
      summary: Enterprise Resource Retrieval 831
      operationId: getAsset831
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 831
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_832:
    get:
      summary: Enterprise Resource Retrieval 832
      operationId: getAsset832
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 832
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_833:
    get:
      summary: Enterprise Resource Retrieval 833
      operationId: getAsset833
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 833
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_834:
    get:
      summary: Enterprise Resource Retrieval 834
      operationId: getAsset834
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 834
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_835:
    get:
      summary: Enterprise Resource Retrieval 835
      operationId: getAsset835
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 835
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_836:
    get:
      summary: Enterprise Resource Retrieval 836
      operationId: getAsset836
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 836
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_837:
    get:
      summary: Enterprise Resource Retrieval 837
      operationId: getAsset837
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 837
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_838:
    get:
      summary: Enterprise Resource Retrieval 838
      operationId: getAsset838
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 838
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_839:
    get:
      summary: Enterprise Resource Retrieval 839
      operationId: getAsset839
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 839
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_840:
    get:
      summary: Enterprise Resource Retrieval 840
      operationId: getAsset840
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 840
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_841:
    get:
      summary: Enterprise Resource Retrieval 841
      operationId: getAsset841
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 841
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_842:
    get:
      summary: Enterprise Resource Retrieval 842
      operationId: getAsset842
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 842
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_843:
    get:
      summary: Enterprise Resource Retrieval 843
      operationId: getAsset843
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 843
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_844:
    get:
      summary: Enterprise Resource Retrieval 844
      operationId: getAsset844
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 844
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_845:
    get:
      summary: Enterprise Resource Retrieval 845
      operationId: getAsset845
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 845
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_846:
    get:
      summary: Enterprise Resource Retrieval 846
      operationId: getAsset846
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 846
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_847:
    get:
      summary: Enterprise Resource Retrieval 847
      operationId: getAsset847
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 847
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_848:
    get:
      summary: Enterprise Resource Retrieval 848
      operationId: getAsset848
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 848
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_849:
    get:
      summary: Enterprise Resource Retrieval 849
      operationId: getAsset849
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 849
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_850:
    get:
      summary: Enterprise Resource Retrieval 850
      operationId: getAsset850
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 850
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_851:
    get:
      summary: Enterprise Resource Retrieval 851
      operationId: getAsset851
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 851
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_852:
    get:
      summary: Enterprise Resource Retrieval 852
      operationId: getAsset852
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 852
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_853:
    get:
      summary: Enterprise Resource Retrieval 853
      operationId: getAsset853
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 853
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_854:
    get:
      summary: Enterprise Resource Retrieval 854
      operationId: getAsset854
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 854
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_855:
    get:
      summary: Enterprise Resource Retrieval 855
      operationId: getAsset855
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 855
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_856:
    get:
      summary: Enterprise Resource Retrieval 856
      operationId: getAsset856
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 856
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_857:
    get:
      summary: Enterprise Resource Retrieval 857
      operationId: getAsset857
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 857
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_858:
    get:
      summary: Enterprise Resource Retrieval 858
      operationId: getAsset858
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 858
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_859:
    get:
      summary: Enterprise Resource Retrieval 859
      operationId: getAsset859
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 859
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_860:
    get:
      summary: Enterprise Resource Retrieval 860
      operationId: getAsset860
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 860
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_861:
    get:
      summary: Enterprise Resource Retrieval 861
      operationId: getAsset861
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 861
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_862:
    get:
      summary: Enterprise Resource Retrieval 862
      operationId: getAsset862
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 862
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_863:
    get:
      summary: Enterprise Resource Retrieval 863
      operationId: getAsset863
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 863
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_864:
    get:
      summary: Enterprise Resource Retrieval 864
      operationId: getAsset864
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 864
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_865:
    get:
      summary: Enterprise Resource Retrieval 865
      operationId: getAsset865
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 865
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_866:
    get:
      summary: Enterprise Resource Retrieval 866
      operationId: getAsset866
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 866
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_867:
    get:
      summary: Enterprise Resource Retrieval 867
      operationId: getAsset867
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 867
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_868:
    get:
      summary: Enterprise Resource Retrieval 868
      operationId: getAsset868
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 868
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_869:
    get:
      summary: Enterprise Resource Retrieval 869
      operationId: getAsset869
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 869
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_870:
    get:
      summary: Enterprise Resource Retrieval 870
      operationId: getAsset870
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 870
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_871:
    get:
      summary: Enterprise Resource Retrieval 871
      operationId: getAsset871
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 871
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_872:
    get:
      summary: Enterprise Resource Retrieval 872
      operationId: getAsset872
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 872
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_873:
    get:
      summary: Enterprise Resource Retrieval 873
      operationId: getAsset873
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 873
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_874:
    get:
      summary: Enterprise Resource Retrieval 874
      operationId: getAsset874
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 874
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_875:
    get:
      summary: Enterprise Resource Retrieval 875
      operationId: getAsset875
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 875
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_876:
    get:
      summary: Enterprise Resource Retrieval 876
      operationId: getAsset876
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 876
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_877:
    get:
      summary: Enterprise Resource Retrieval 877
      operationId: getAsset877
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 877
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_878:
    get:
      summary: Enterprise Resource Retrieval 878
      operationId: getAsset878
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 878
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_879:
    get:
      summary: Enterprise Resource Retrieval 879
      operationId: getAsset879
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 879
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_880:
    get:
      summary: Enterprise Resource Retrieval 880
      operationId: getAsset880
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 880
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_881:
    get:
      summary: Enterprise Resource Retrieval 881
      operationId: getAsset881
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 881
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_882:
    get:
      summary: Enterprise Resource Retrieval 882
      operationId: getAsset882
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 882
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_883:
    get:
      summary: Enterprise Resource Retrieval 883
      operationId: getAsset883
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 883
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_884:
    get:
      summary: Enterprise Resource Retrieval 884
      operationId: getAsset884
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 884
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_885:
    get:
      summary: Enterprise Resource Retrieval 885
      operationId: getAsset885
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 885
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_886:
    get:
      summary: Enterprise Resource Retrieval 886
      operationId: getAsset886
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 886
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_887:
    get:
      summary: Enterprise Resource Retrieval 887
      operationId: getAsset887
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 887
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_888:
    get:
      summary: Enterprise Resource Retrieval 888
      operationId: getAsset888
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 888
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_889:
    get:
      summary: Enterprise Resource Retrieval 889
      operationId: getAsset889
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 889
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_890:
    get:
      summary: Enterprise Resource Retrieval 890
      operationId: getAsset890
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 890
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_891:
    get:
      summary: Enterprise Resource Retrieval 891
      operationId: getAsset891
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 891
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_892:
    get:
      summary: Enterprise Resource Retrieval 892
      operationId: getAsset892
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 892
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_893:
    get:
      summary: Enterprise Resource Retrieval 893
      operationId: getAsset893
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 893
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_894:
    get:
      summary: Enterprise Resource Retrieval 894
      operationId: getAsset894
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 894
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_895:
    get:
      summary: Enterprise Resource Retrieval 895
      operationId: getAsset895
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 895
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_896:
    get:
      summary: Enterprise Resource Retrieval 896
      operationId: getAsset896
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 896
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_897:
    get:
      summary: Enterprise Resource Retrieval 897
      operationId: getAsset897
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 897
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_898:
    get:
      summary: Enterprise Resource Retrieval 898
      operationId: getAsset898
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 898
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_899:
    get:
      summary: Enterprise Resource Retrieval 899
      operationId: getAsset899
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 899
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_900:
    get:
      summary: Enterprise Resource Retrieval 900
      operationId: getAsset900
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 900
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_901:
    get:
      summary: Enterprise Resource Retrieval 901
      operationId: getAsset901
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 901
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_902:
    get:
      summary: Enterprise Resource Retrieval 902
      operationId: getAsset902
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 902
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_903:
    get:
      summary: Enterprise Resource Retrieval 903
      operationId: getAsset903
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 903
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_904:
    get:
      summary: Enterprise Resource Retrieval 904
      operationId: getAsset904
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 904
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_905:
    get:
      summary: Enterprise Resource Retrieval 905
      operationId: getAsset905
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 905
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_906:
    get:
      summary: Enterprise Resource Retrieval 906
      operationId: getAsset906
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 906
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_907:
    get:
      summary: Enterprise Resource Retrieval 907
      operationId: getAsset907
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 907
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_908:
    get:
      summary: Enterprise Resource Retrieval 908
      operationId: getAsset908
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 908
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_909:
    get:
      summary: Enterprise Resource Retrieval 909
      operationId: getAsset909
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 909
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_910:
    get:
      summary: Enterprise Resource Retrieval 910
      operationId: getAsset910
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 910
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_911:
    get:
      summary: Enterprise Resource Retrieval 911
      operationId: getAsset911
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 911
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_912:
    get:
      summary: Enterprise Resource Retrieval 912
      operationId: getAsset912
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 912
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_913:
    get:
      summary: Enterprise Resource Retrieval 913
      operationId: getAsset913
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 913
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_914:
    get:
      summary: Enterprise Resource Retrieval 914
      operationId: getAsset914
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 914
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_915:
    get:
      summary: Enterprise Resource Retrieval 915
      operationId: getAsset915
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 915
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_916:
    get:
      summary: Enterprise Resource Retrieval 916
      operationId: getAsset916
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 916
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_917:
    get:
      summary: Enterprise Resource Retrieval 917
      operationId: getAsset917
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 917
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_918:
    get:
      summary: Enterprise Resource Retrieval 918
      operationId: getAsset918
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 918
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_919:
    get:
      summary: Enterprise Resource Retrieval 919
      operationId: getAsset919
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 919
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_920:
    get:
      summary: Enterprise Resource Retrieval 920
      operationId: getAsset920
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 920
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_921:
    get:
      summary: Enterprise Resource Retrieval 921
      operationId: getAsset921
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 921
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_922:
    get:
      summary: Enterprise Resource Retrieval 922
      operationId: getAsset922
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 922
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_923:
    get:
      summary: Enterprise Resource Retrieval 923
      operationId: getAsset923
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 923
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_924:
    get:
      summary: Enterprise Resource Retrieval 924
      operationId: getAsset924
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 924
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_925:
    get:
      summary: Enterprise Resource Retrieval 925
      operationId: getAsset925
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 925
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_926:
    get:
      summary: Enterprise Resource Retrieval 926
      operationId: getAsset926
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 926
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_927:
    get:
      summary: Enterprise Resource Retrieval 927
      operationId: getAsset927
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 927
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_928:
    get:
      summary: Enterprise Resource Retrieval 928
      operationId: getAsset928
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 928
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_929:
    get:
      summary: Enterprise Resource Retrieval 929
      operationId: getAsset929
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 929
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_930:
    get:
      summary: Enterprise Resource Retrieval 930
      operationId: getAsset930
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 930
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_931:
    get:
      summary: Enterprise Resource Retrieval 931
      operationId: getAsset931
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 931
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_932:
    get:
      summary: Enterprise Resource Retrieval 932
      operationId: getAsset932
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 932
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_933:
    get:
      summary: Enterprise Resource Retrieval 933
      operationId: getAsset933
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 933
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_934:
    get:
      summary: Enterprise Resource Retrieval 934
      operationId: getAsset934
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 934
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_935:
    get:
      summary: Enterprise Resource Retrieval 935
      operationId: getAsset935
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 935
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_936:
    get:
      summary: Enterprise Resource Retrieval 936
      operationId: getAsset936
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 936
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_937:
    get:
      summary: Enterprise Resource Retrieval 937
      operationId: getAsset937
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 937
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_938:
    get:
      summary: Enterprise Resource Retrieval 938
      operationId: getAsset938
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 938
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_939:
    get:
      summary: Enterprise Resource Retrieval 939
      operationId: getAsset939
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 939
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_940:
    get:
      summary: Enterprise Resource Retrieval 940
      operationId: getAsset940
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 940
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_941:
    get:
      summary: Enterprise Resource Retrieval 941
      operationId: getAsset941
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 941
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_942:
    get:
      summary: Enterprise Resource Retrieval 942
      operationId: getAsset942
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 942
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_943:
    get:
      summary: Enterprise Resource Retrieval 943
      operationId: getAsset943
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 943
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_944:
    get:
      summary: Enterprise Resource Retrieval 944
      operationId: getAsset944
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 944
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_945:
    get:
      summary: Enterprise Resource Retrieval 945
      operationId: getAsset945
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 945
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_946:
    get:
      summary: Enterprise Resource Retrieval 946
      operationId: getAsset946
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 946
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_947:
    get:
      summary: Enterprise Resource Retrieval 947
      operationId: getAsset947
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 947
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_948:
    get:
      summary: Enterprise Resource Retrieval 948
      operationId: getAsset948
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 948
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_949:
    get:
      summary: Enterprise Resource Retrieval 949
      operationId: getAsset949
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 949
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_950:
    get:
      summary: Enterprise Resource Retrieval 950
      operationId: getAsset950
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 950
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_951:
    get:
      summary: Enterprise Resource Retrieval 951
      operationId: getAsset951
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 951
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_952:
    get:
      summary: Enterprise Resource Retrieval 952
      operationId: getAsset952
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 952
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_953:
    get:
      summary: Enterprise Resource Retrieval 953
      operationId: getAsset953
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 953
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_954:
    get:
      summary: Enterprise Resource Retrieval 954
      operationId: getAsset954
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 954
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_955:
    get:
      summary: Enterprise Resource Retrieval 955
      operationId: getAsset955
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 955
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_956:
    get:
      summary: Enterprise Resource Retrieval 956
      operationId: getAsset956
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 956
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_957:
    get:
      summary: Enterprise Resource Retrieval 957
      operationId: getAsset957
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 957
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_958:
    get:
      summary: Enterprise Resource Retrieval 958
      operationId: getAsset958
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 958
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_959:
    get:
      summary: Enterprise Resource Retrieval 959
      operationId: getAsset959
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 959
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_960:
    get:
      summary: Enterprise Resource Retrieval 960
      operationId: getAsset960
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 960
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_961:
    get:
      summary: Enterprise Resource Retrieval 961
      operationId: getAsset961
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 961
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_962:
    get:
      summary: Enterprise Resource Retrieval 962
      operationId: getAsset962
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 962
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_963:
    get:
      summary: Enterprise Resource Retrieval 963
      operationId: getAsset963
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 963
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_964:
    get:
      summary: Enterprise Resource Retrieval 964
      operationId: getAsset964
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 964
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_965:
    get:
      summary: Enterprise Resource Retrieval 965
      operationId: getAsset965
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 965
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_966:
    get:
      summary: Enterprise Resource Retrieval 966
      operationId: getAsset966
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 966
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_967:
    get:
      summary: Enterprise Resource Retrieval 967
      operationId: getAsset967
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 967
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_968:
    get:
      summary: Enterprise Resource Retrieval 968
      operationId: getAsset968
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 968
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_969:
    get:
      summary: Enterprise Resource Retrieval 969
      operationId: getAsset969
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 969
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_970:
    get:
      summary: Enterprise Resource Retrieval 970
      operationId: getAsset970
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 970
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_971:
    get:
      summary: Enterprise Resource Retrieval 971
      operationId: getAsset971
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 971
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_972:
    get:
      summary: Enterprise Resource Retrieval 972
      operationId: getAsset972
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 972
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_973:
    get:
      summary: Enterprise Resource Retrieval 973
      operationId: getAsset973
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 973
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_974:
    get:
      summary: Enterprise Resource Retrieval 974
      operationId: getAsset974
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 974
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_975:
    get:
      summary: Enterprise Resource Retrieval 975
      operationId: getAsset975
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 975
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_976:
    get:
      summary: Enterprise Resource Retrieval 976
      operationId: getAsset976
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 976
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_977:
    get:
      summary: Enterprise Resource Retrieval 977
      operationId: getAsset977
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 977
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_978:
    get:
      summary: Enterprise Resource Retrieval 978
      operationId: getAsset978
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 978
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_979:
    get:
      summary: Enterprise Resource Retrieval 979
      operationId: getAsset979
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 979
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_980:
    get:
      summary: Enterprise Resource Retrieval 980
      operationId: getAsset980
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 980
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_981:
    get:
      summary: Enterprise Resource Retrieval 981
      operationId: getAsset981
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 981
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_982:
    get:
      summary: Enterprise Resource Retrieval 982
      operationId: getAsset982
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 982
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_983:
    get:
      summary: Enterprise Resource Retrieval 983
      operationId: getAsset983
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 983
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_984:
    get:
      summary: Enterprise Resource Retrieval 984
      operationId: getAsset984
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 984
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_985:
    get:
      summary: Enterprise Resource Retrieval 985
      operationId: getAsset985
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 985
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_986:
    get:
      summary: Enterprise Resource Retrieval 986
      operationId: getAsset986
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 986
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_987:
    get:
      summary: Enterprise Resource Retrieval 987
      operationId: getAsset987
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 987
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_988:
    get:
      summary: Enterprise Resource Retrieval 988
      operationId: getAsset988
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 988
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_989:
    get:
      summary: Enterprise Resource Retrieval 989
      operationId: getAsset989
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 989
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_990:
    get:
      summary: Enterprise Resource Retrieval 990
      operationId: getAsset990
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 990
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_991:
    get:
      summary: Enterprise Resource Retrieval 991
      operationId: getAsset991
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 991
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_992:
    get:
      summary: Enterprise Resource Retrieval 992
      operationId: getAsset992
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 992
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_993:
    get:
      summary: Enterprise Resource Retrieval 993
      operationId: getAsset993
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 993
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_994:
    get:
      summary: Enterprise Resource Retrieval 994
      operationId: getAsset994
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 994
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_995:
    get:
      summary: Enterprise Resource Retrieval 995
      operationId: getAsset995
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 995
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_996:
    get:
      summary: Enterprise Resource Retrieval 996
      operationId: getAsset996
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 996
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_997:
    get:
      summary: Enterprise Resource Retrieval 997
      operationId: getAsset997
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 997
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_998:
    get:
      summary: Enterprise Resource Retrieval 998
      operationId: getAsset998
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 998
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_999:
    get:
      summary: Enterprise Resource Retrieval 999
      operationId: getAsset999
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 999
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error

  /v2/enterprise/resource/asset_1000:
    get:
      summary: Enterprise Resource Retrieval 1000
      operationId: getAsset1000
      security:
        - OAuth2: [read]
      responses:
        '200':
          description: Successfully retrieved asset 1000
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  timestamp:
                    type: string
                    format: date-time
                  payload:
                    type: object
                    additionalProperties: true
        '400':
          description: Bad Request
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '429':
          description: Rate Limit Exceeded
        '500':
          description: Internal Server Error
```