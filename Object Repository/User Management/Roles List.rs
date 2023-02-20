<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
<<<<<<<< HEAD:Object Repository/User Management/Roles List.rs
   <name>Roles List</name>
   <tag></tag>
   <elementGuidId>6b3fc3c3-be98-4c67-a4c7-b96a15a47569</elementGuidId>
========
   <name>ListProjectBaseVersion</name>
   <tag></tag>
   <elementGuidId>65bb7dad-3579-47fa-af72-3223168405f7</elementGuidId>
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProjectBaseVersion.rs
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
<<<<<<<< HEAD:Object Repository/User Management/Roles List.rs
  &quot;text&quot;: &quot;{\n  \&quot;roleLevel\&quot;: \&quot;project\&quot;\n}&quot;,
========
  &quot;text&quot;: &quot;{\n\t\&quot;projectBase\&quot;:\&quot;Sitecore\&quot;\n}&quot;,
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProjectBaseVersion.rs
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
<<<<<<<< HEAD:Object Repository/User Management/Roles List.rs
      <webElementGuid>ac11af7d-8eee-4a5a-ae6b-bc2e38a3250a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>152eb7b9-7285-4a8e-ac68-64dde7ce3ae0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL}api/usermanagement/v1/roles/list</restUrl>
========
      <webElementGuid>074453b3-49f4-468f-a49a-a21b0f10740a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL}api/ProjectManagement/v1/project/listProjectBaseVersion</restUrl>
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Object Repository/Project Management/ListProjectBaseVersion.rs
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
