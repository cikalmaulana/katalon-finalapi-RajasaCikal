<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create Booking</name>
   <tag></tag>
   <elementGuidId>f821954e-2ae1-41fa-a5ff-b4a0e3fa32b2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${GlobalVariable.firstname}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${GlobalVariable.lastname}\&quot;,\n    \&quot;totalprice\&quot; : \&quot;${GlobalVariable.totalprice}\&quot;,\n    \&quot;depositpaid\&quot; : \&quot;${GlobalVariable.depositpaid}\&quot;,\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;2024-02-14\&quot;,\n        \&quot;checkout\&quot; : \&quot;2024-02-15\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${GlobalVariable.additionalneeds}\&quot;\n}\n&quot;,
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
      <webElementGuid>e90bb74e-7ee0-421f-aa0f-9e6d96fac35d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d99a3789-4429-479e-9f7d-c93264720569</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, &quot;booking.firstname&quot;, &quot;Dika&quot;)
WS.verifyElementPropertyValue(response, &quot;booking.lastname&quot;, &quot;Naga&quot;)
WS.verifyElementPropertyValue(response, &quot;booking.totalprice&quot;, &quot;606&quot;)
WS.verifyElementPropertyValue(response, &quot;booking.depositpaid&quot;, true)
WS.verifyElementPropertyValue(response, &quot;booking.bookingdates.checkin&quot;, &quot;2024-02-14&quot;)
WS.verifyElementPropertyValue(response, &quot;booking.bookingdates.checkout&quot;, &quot;2024-02-15&quot;)
WS.verifyElementPropertyValue(response, &quot;booking.additionalneeds&quot;, &quot;No need&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
