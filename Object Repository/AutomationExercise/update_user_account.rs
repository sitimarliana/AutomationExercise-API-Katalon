<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update_user_account</name>
   <tag></tag>
   <elementGuidId>578ff235-0183-40da-80a8-cebfa3248c78</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;sitiupdate&quot;
    },
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;sitimarliana@gmail.com&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;12345&quot;
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
      &quot;value&quot;: &quot;Mar&quot;
    },
    {
      &quot;name&quot;: &quot;lastname&quot;,
      &quot;value&quot;: &quot;Liana&quot;
    },
    {
      &quot;name&quot;: &quot;company&quot;,
      &quot;value&quot;: &quot;bintang&quot;
    },
    {
      &quot;name&quot;: &quot;address1&quot;,
      &quot;value&quot;: &quot;jalan bulan&quot;
    },
    {
      &quot;name&quot;: &quot;address2&quot;,
      &quot;value&quot;: &quot;jalan bintang&quot;
    },
    {
      &quot;name&quot;: &quot;country&quot;,
      &quot;value&quot;: &quot;indonesia&quot;
    },
    {
      &quot;name&quot;: &quot;zipcode&quot;,
      &quot;value&quot;: &quot;62344&quot;
    },
    {
      &quot;name&quot;: &quot;state&quot;,
      &quot;value&quot;: &quot;sleman&quot;
    },
    {
      &quot;name&quot;: &quot;city&quot;,
      &quot;value&quot;: &quot;yogyakarta&quot;
    },
    {
      &quot;name&quot;: &quot;mobile_number&quot;,
      &quot;value&quot;: &quot;08976555444&quot;
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
      <webElementGuid>bf85ba3b-b8e7-4dd6-a8d7-4018d46b7a08</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/updateAccount</restUrl>
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
