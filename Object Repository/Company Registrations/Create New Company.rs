<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create New Company</name>
   <tag></tag>
   <elementGuidId>12929fbe-9e53-491a-bc56-3a48bb134cdd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;company\&quot;: {\n        \&quot;name\&quot;: \&quot;AutoTesting_SDET\&quot;,\n        \&quot;address_1\&quot;: null,\n        \&quot;address_2\&quot;: null,\n        \&quot;province\&quot;: null,\n        \&quot;city\&quot;: null,\n        \&quot;country_code\&quot;: \&quot;ID\&quot;,\n        \&quot;country_name\&quot;: \&quot;Indonesia\&quot;,\n        \&quot;postal_code\&quot;: null,\n        \&quot;phone\&quot;: null,\n        \&quot;email\&quot;: null,\n        \&quot;website\&quot;: null\n    },\n    \&quot;setup\&quot;: {\n        \&quot;business_id\&quot;: \&quot;e005a086-c2fa-4aa1-81eb-0e1add142e69\&quot;,\n        \&quot;business_name\&quot;: \&quot;Online Shop\&quot;,\n        \&quot;business_type\&quot;: 1001,\n        \&quot;business_type_name\&quot;: \&quot;General Trading\&quot;,\n        \&quot;is_use_default_coa\&quot;: true,\n        \&quot;privacy_policy\&quot;: true,\n        \&quot;currency_code\&quot;: \&quot;IDR\&quot;,\n        \&quot;language_code\&quot;: \&quot;en\&quot;,\n        \&quot;tax_code\&quot;: \&quot;id\&quot;,\n        \&quot;period\&quot;: {\n            \&quot;start\&quot;: \&quot;3\&quot;,\n            \&quot;end\&quot;: \&quot;12\&quot;,\n            \&quot;year\&quot;: 2020\n        }\n    },\n    \&quot;date_format\&quot;: {\n        \&quot;time_zone\&quot;: null,\n        \&quot;short\&quot;: null,\n        \&quot;medium\&quot;: null,\n        \&quot;long\&quot;: null\n    },\n    \&quot;number_format\&quot;: {\n        \&quot;thousand_separator\&quot;: \&quot;,\&quot;,\n        \&quot;decimal_separator\&quot;: \&quot;.\&quot;,\n        \&quot;round_digit\&quot;: 2\n    },\n    \&quot;membership\&quot;: {\n        \&quot;variant_id\&quot;: 4,\n        \&quot;referral_code\&quot;: null\n    }\n}&quot;,
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
      <value>Bearer zaJ33iqpG2bCXkTyg0wL5xgMD8FrK7cwGTVKgVkg</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v2/company_registrations</restUrl>
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
      <id>fee5bc81-0869-4ec5-8a4b-c226a7ce83ec</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



def variables = request.getVariables(response.access_token)
def variable = variables.get('access_token')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
