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

WebUI.callTestCase(findTestCase('POST/TC - POST Get Token'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('POST/TC - POST Create Booking'), [:], FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('PUT Create Booking', [('bookingid') : GlobalVariable.bookingid]))

String firstname = WS.getElementPropertyValue(response, "firstname")
println(firstname)

String gbFirstName = GlobalVariable.firstname
println(gbFirstName)

WS.verifyElementPropertyValue(response, "firstname", GlobalVariable.firstname)
WS.verifyElementPropertyValue(response, "lastname", GlobalVariable.lastname)
WS.verifyElementPropertyValue(response, "totalprice", GlobalVariable.totalprice)
WS.verifyElementPropertyValue(response, "depositpaid", GlobalVariable.depositpaid)
WS.verifyElementPropertyValue(response, "bookingdates.checkin", "2024-02-14")
WS.verifyElementPropertyValue(response, "bookingdates.checkout", "2024-02-15")
WS.verifyElementPropertyValue(response, "additionalneeds", GlobalVariable.additionalneeds)