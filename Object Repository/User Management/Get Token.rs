<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
<<<<<<<< HEAD:Object Repository/User Management/Get Token.rs
   <name>Get Token</name>
   <tag></tag>
   <elementGuidId>50fe7395-2c0b-492b-b862-100717a5a6b0</elementGuidId>
========
   <name>ListProject</name>
   <tag></tag>
   <elementGuidId>8d9d2fa0-19c3-46ce-98ff-8f1151c0b8fa</elementGuidId>
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProject.rs
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
<<<<<<<< HEAD:Object Repository/User Management/Get Token.rs
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;client_secret&quot;,
      &quot;value&quot;: &quot;dE_CBiYw_tisz8J1&quot;
    },
    {
      &quot;name&quot;: &quot;client_id&quot;,
      &quot;value&quot;: &quot;backend-service&quot;
    },
    {
      &quot;name&quot;: &quot;grant_type&quot;,
      &quot;value&quot;: &quot;client_credentials&quot;
    }
  ]
========
  &quot;text&quot;: &quot;{\n    \&quot;quickSearch\&quot;:\&quot;\&quot;,\n    \&quot;TenantCode\&quot;:\&quot;MOE001PRO1\&quot;,\n    \&quot;Page\&quot;:1,\n    \&quot;PageSize\&quot;:10,\n    \&quot;Sort\&quot;:\&quot;ProjectCode\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProject.rs
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
<<<<<<<< HEAD:Object Repository/User Management/Get Token.rs
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>f8027ab2-ad7e-4c0d-bbc8-c43c43c09bc3</webElementGuid>
========
      <value>application/json</value>
      <webElementGuid>ae5c0265-1812-43b4-a675-4a5bb14bd321</webElementGuid>
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProject.rs
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
<<<<<<<< HEAD:Object Repository/User Management/Get Token.rs
   <restUrl>https://app-dev.cloudvanti.com/api/base/idp/connect/token</restUrl>
========
   <restUrl>${GlobalVariable.BASE_URL}api/ProjectManagement/v1/project/listProject</restUrl>
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProject.rs
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
