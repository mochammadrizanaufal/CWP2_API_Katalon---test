
package common

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper

public class Common {

	public static JsonSlurper jsonSlurper = new JsonSlurper()

	@Keyword
	def getToken() {
		def response = WS.sendRequestAndVerify(findTestObject("User Management/Get Token"))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		return jsonResponse.access_token
	}
}
