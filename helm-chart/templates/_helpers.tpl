{{/*
Expand the name of the chart.
*/}}
{{- define "pw.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "pw.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "pw.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "pw.labels" -}}
helm.sh/chart: {{ include "pw.chart" . }}
{{ include "pw.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- with .Values.labels }}
{{ toYaml . }}
{{- end }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "pw.selectorLabels" -}}
app.kubernetes.io/name: {{ include "pw.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
PW selector labels
*/}}
{{- define "pw.pwSelectorLabels" -}}
{{ include "pw.selectorLabels" . }}
app.kubernetes.io/component: pw
{{- end }}

{{/*
Redis selector labels
*/}}
{{- define "pw.redisSelectorLabels" -}}
{{ include "pw.selectorLabels" . }}
app.kubernetes.io/component: redis
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "pw.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "pw.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Metrics service name
*/}}
{{- define "pw.metricsServiceName" -}}
{{- printf "%s-pw-metrics" (include "pw.fullname" .) }}
{{- end }}

{{/*
Redis service name
*/}}
{{- define "pw.redisServiceName" -}}
{{- printf "%s-redis" (include "pw.fullname" .) }}
{{- end }}

{{/*
Redis secret name
*/}}
{{- define "pw.redisSecretName" -}}
{{- printf "%s-redis-auth" (include "pw.fullname" .) }}
{{- end }}

{{/*
Redis connection URL
*/}}
{{- define "pw.redisUrl" -}}
{{- if .Values.redis.auth.enabled }}
{{- printf "redis://:%s@%s:%d/" "${REDIS_PASSWORD}" (include "pw.redisServiceName" .) (.Values.redis.service.port | int) }}
{{- else }}
{{- printf "redis://%s:%d/" (include "pw.redisServiceName" .) (.Values.redis.service.port | int) }}
{{- end }}
{{- end }}

{{/*
Generate Redis password
*/}}
{{- define "pw.redisPassword" -}}
{{- if .Values.redis.auth.password }}
{{- .Values.redis.auth.password }}
{{- else }}
{{- randAlphaNum 16 }}
{{- end }}
{{- end }}
