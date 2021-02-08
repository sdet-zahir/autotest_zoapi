<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Contacts</name>
   <tag></tag>
   <elementGuidId>c78dcba8-45c9-4b5f-a883-126876889c0e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;code\&quot;: \&quot;C-3\&quot;,\n\t\&quot;name\&quot;: \&quot;Cust - 2\&quot;,\n\t\&quot;is_customer\&quot;: true,\n\t\&quot;is_supplier\&quot;: true,\n\t\&quot;is_employee\&quot;: true,\n\t\&quot;is_active\&quot;: true,\n\t\&quot;credit_limit\&quot;: 100000000,\n\t\&quot;term_of_payment\&quot;: {\n\t    \&quot;due_days\&quot;: 0,\n\t    \&quot;late_charge\&quot;: 0,\n\t    \&quot;discount_days\&quot;: 0,\n\t    \&quot;early_discount\&quot;: 0\n\t},\n\t\&quot;classification\&quot;: {\n\t    \&quot;id\&quot;: \&quot;{{contact_classifications.id}}\&quot;\n\t},\n\t\&quot;addresses\&quot;: [],\n\t\&quot;contact_person\&quot;: [],\n\t\&quot;phones\&quot;: [],\n\t\&quot;emails\&quot;: [],\n\t\&quot;other_fields\&quot;: [],\n\t\&quot;attachments\&quot;: []\n\t\n}&quot;,
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
   <katalonVersion>7.9.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v2/contacts</restUrl>
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
      <id>0ced8591-abe9-427e-a1ca-43cbfef621df</id>
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
