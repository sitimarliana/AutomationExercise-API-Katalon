<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post_register</name>
   <tag></tag>
   <elementGuidId>e1219396-4266-40e7-a4d5-39308132c188</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;simar&quot;
    },
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;sitimarliana20@gmail.com&quot;
    },
    {
      &quot;name&quot;: &quot;title&quot;,
      &quot;value&quot;: &quot;Miss&quot;
    },
    {
      &quot;name&quot;: &quot;birth_date&quot;,
      &quot;value&quot;: &quot;18&quot;
    },
    {
      &quot;name&quot;: &quot;birth_month&quot;,
      &quot;value&quot;: &quot;09&quot;
    },
    {
      &quot;name&quot;: &quot;birth_year&quot;,
      &quot;value&quot;: &quot;2000&quot;
    },
    {
      &quot;name&quot;: &quot;firstname&quot;,
      &quot;value&quot;: &quot;siti&quot;
    },
    {
      &quot;name&quot;: &quot;lastname&quot;,
      &quot;value&quot;: &quot;marliana&quot;
    },
    {
      &quot;name&quot;: &quot;company&quot;,
      &quot;value&quot;: &quot;bintang&quot;
    },
    {
      &quot;name&quot;: &quot;address1&quot;,
      &quot;value&quot;: &quot;Jalan kanangan&quot;
    },
    {
      &quot;name&quot;: &quot;address2&quot;,
      &quot;value&quot;: &quot;Jalan merapti&quot;
    },
    {
      &quot;name&quot;: &quot;country&quot;,
      &quot;value&quot;: &quot;Yogyakarta&quot;
    },
    {
      &quot;name&quot;: &quot;zipcode&quot;,
      &quot;value&quot;: &quot;56777&quot;
    },
    {
      &quot;name&quot;: &quot;state&quot;,
      &quot;value&quot;: &quot;3444&quot;
    },
    {
      &quot;name&quot;: &quot;city&quot;,
      &quot;value&quot;: &quot;sleman&quot;
    },
    {
      &quot;name&quot;: &quot;mobile_number&quot;,
      &quot;value&quot;: &quot;0897655566677&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;12345&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>238407a2-f5c5-434a-b784-09706a9b92b2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/createAccount</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
