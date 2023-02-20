import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

<<<<<<<< HEAD:Scripts/User Management/Verify Success Code - Users Filter/Script1676432703671.groovy
def token = CustomKeywords.'common.Common.getToken'()
========
request = WS.sendRequest(findTestObject('Project Management/GetProject'))
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Scripts/Project Management/Verify Success Code - GetProject/Script1676435484547.groovy

result = WS.sendRequest(findTestObject('User Management/Users Filter', [('token'):token]))

<<<<<<<< HEAD:Scripts/User Management/Verify Success Code - Users Filter/Script1676432703671.groovy
WS.verifyResponseStatusCode(result, GlobalVariable.SUCCESS_CODE)

========
>>>>>>>> 9a4402edd71613e544bf76afeee04521f3490a35:Scripts/Project Management/Verify Success Code - GetProject/Script1676435484547.groovy
