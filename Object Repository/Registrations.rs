<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Registrations</name>
   <tag></tag>
   <elementGuidId>e74e163e-2adb-4a99-a852-4ca1e5fc2c3a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;user\&quot;: {\n\t    \&quot;email\&quot;: \&quot;kaka.tori09@gmail.com\&quot;,\n\t    \&quot;password\&quot;: \&quot;k4k4t0r1\&quot;,\n\t    \&quot;first_name\&quot;: \&quot;Testing\&quot;,\n\t    \&quot;last_name\&quot;: \&quot;ZO\&quot;,\n\t    \&quot;mobile_phone\&quot;: \&quot;099999999\&quot;\n\t},\n\t \&quot;company\&quot;: {\n        \&quot;name\&quot;: \&quot;PT ZAHIR\&quot;,\n        \&quot;address_1\&quot;: \&quot;Jl.H.Abu No.23\&quot;,\n        \&quot;address_2\&quot;: \&quot;Jl.Kemang Selatan no.20D\&quot;,\n        \&quot;city\&quot;: \&quot;Jakarta Selatan\&quot;,\n        \&quot;province\&quot;: \&quot;DKI JAKARTA\&quot;,\n        \&quot;country_code\&quot;: \&quot;id\&quot;,\n        \&quot;country_name\&quot;: \&quot;Indonesia\&quot;,\n        \&quot;postal_code\&quot;: \&quot;17412\&quot;,\n        \&quot;phone\&quot;: \&quot;021898989898\&quot;,\n        \&quot;email\&quot;: \&quot;zahir@gmail.com\&quot;,\n        \&quot;website\&quot;: \&quot;go.zahironline.com\&quot;\n    },\n    \&quot;setup\&quot;: {\n    \t\&quot;business_type\&quot;: 1001,\n    \t\&quot;business_type_name\&quot;: \&quot;General Trading\&quot;,\n\t    \&quot;currency_code\&quot;: \&quot;IDR\&quot;,\n\t    \&quot;language_code\&quot;: \&quot;id\&quot;,\n\t    \&quot;tax_code\&quot;: \&quot;ID\&quot;\n\t},\n    \&quot;membership\&quot;: {\n    \t\&quot;variant_id\&quot;: 4\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>3bWqNlzxxqlp2K1m1XsD2aYv4A6pa9jBPxReCi1y</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v2/registrations</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_url</defaultValue>
      <description></description>
      <id>e68104ea-887f-46d0-b249-678cdd1b2d0a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
